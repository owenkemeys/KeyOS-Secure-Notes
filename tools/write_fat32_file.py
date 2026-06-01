#!/usr/bin/env python3
import argparse
import math
import os
import struct

EOC = 0x0FFFFFFF


class Fat32Image:
    def __init__(self, image_path):
        self.image_path = image_path
        self.image = open(image_path, "r+b")
        boot = self.read_at(0, 512)
        self.bytes_per_sector = struct.unpack_from("<H", boot, 11)[0]
        self.sectors_per_cluster = boot[13]
        self.reserved_sectors = struct.unpack_from("<H", boot, 14)[0]
        self.fat_count = boot[16]
        self.fat_size_sectors = struct.unpack_from("<I", boot, 36)[0]
        self.root_cluster = struct.unpack_from("<I", boot, 44)[0]
        self.cluster_size = self.bytes_per_sector * self.sectors_per_cluster
        self.fat_offset = self.reserved_sectors * self.bytes_per_sector
        self.data_offset = (
            self.reserved_sectors + self.fat_count * self.fat_size_sectors
        ) * self.bytes_per_sector
        self.fat_entries = self.fat_size_sectors * self.bytes_per_sector // 4

    def close(self):
        self.image.flush()
        os.fsync(self.image.fileno())
        self.image.close()

    def read_at(self, offset, size):
        self.image.seek(offset)
        return self.image.read(size)

    def write_at(self, offset, data):
        self.image.seek(offset)
        self.image.write(data)

    def cluster_offset(self, cluster):
        return self.data_offset + (cluster - 2) * self.cluster_size

    def read_cluster(self, cluster):
        return self.read_at(self.cluster_offset(cluster), self.cluster_size)

    def write_cluster(self, cluster, data):
        payload = data[: self.cluster_size]
        payload += b"\0" * (self.cluster_size - len(payload))
        self.write_at(self.cluster_offset(cluster), payload)

    def fat_entry(self, cluster):
        raw = self.read_at(self.fat_offset + cluster * 4, 4)
        return struct.unpack("<I", raw)[0] & 0x0FFFFFFF

    def set_fat_entry(self, cluster, value):
        data = struct.pack("<I", value & 0x0FFFFFFF)
        for fat_index in range(self.fat_count):
            offset = (
                self.fat_offset
                + fat_index * self.fat_size_sectors * self.bytes_per_sector
                + cluster * 4
            )
            self.write_at(offset, data)

    def cluster_chain(self, start):
        chain = []
        cluster = start
        while 2 <= cluster < EOC:
            chain.append(cluster)
            next_cluster = self.fat_entry(cluster)
            if next_cluster >= 0x0FFFFFF8:
                break
            cluster = next_cluster
        return chain

    def allocate_clusters(self, count):
        allocated = []
        for cluster in range(2, self.fat_entries):
            if self.fat_entry(cluster) == 0:
                allocated.append(cluster)
                if len(allocated) == count:
                    break
        if len(allocated) != count:
            raise RuntimeError("Not enough free clusters in image")
        for index, cluster in enumerate(allocated):
            next_cluster = allocated[index + 1] if index + 1 < len(allocated) else EOC
            self.set_fat_entry(cluster, next_cluster)
            self.write_cluster(cluster, b"")
        return allocated

    def free_chain(self, start):
        if start < 2:
            return
        for cluster in self.cluster_chain(start):
            self.set_fat_entry(cluster, 0)

    def directory_entries(self, dir_cluster):
        for cluster in self.cluster_chain(dir_cluster):
            data = self.read_cluster(cluster)
            for offset in range(0, self.cluster_size, 32):
                entry = data[offset : offset + 32]
                yield cluster, offset, entry
                if entry[0] == 0x00:
                    return

    def find_entry(self, dir_cluster, name83):
        for cluster, offset, entry in self.directory_entries(dir_cluster):
            if entry[0] in (0x00, 0xE5) or entry[11] == 0x0F:
                continue
            if entry[:11] == name83:
                return cluster, offset, entry
        return None

    def find_free_entry(self, dir_cluster):
        return self.find_free_entries(dir_cluster, 1)[0]

    def find_free_entries(self, dir_cluster, count):
        chain = self.cluster_chain(dir_cluster)
        for cluster in chain:
            data = self.read_cluster(cluster)
            run = []
            for offset in range(0, self.cluster_size, 32):
                if data[offset] in (0x00, 0xE5):
                    run.append((cluster, offset))
                    if len(run) == count:
                        return run
                else:
                    run = []
        new_cluster = self.allocate_clusters(1)[0]
        self.set_fat_entry(chain[-1], new_cluster)
        self.set_fat_entry(new_cluster, EOC)
        return [(new_cluster, offset) for offset in range(0, count * 32, 32)]

    def write_entry(self, cluster, offset, entry):
        self.write_at(self.cluster_offset(cluster) + offset, entry)

    def mark_entry_deleted(self, cluster, offset):
        self.write_at(self.cluster_offset(cluster) + offset, b"\xE5")

    def delete_entry_and_lfns(self, dir_cluster, target_cluster, target_offset):
        entries = list(self.directory_entries(dir_cluster))
        target_index = None
        for index, (cluster, offset, _entry) in enumerate(entries):
            if cluster == target_cluster and offset == target_offset:
                target_index = index
                break
        if target_index is None:
            return

        self.mark_entry_deleted(target_cluster, target_offset)
        index = target_index - 1
        while index >= 0:
            cluster, offset, entry = entries[index]
            if entry[11] != 0x0F:
                break
            self.mark_entry_deleted(cluster, offset)
            index -= 1

    def ensure_dir(self, parent_cluster, name):
        name83 = to_83(name)
        existing = self.find_entry(parent_cluster, name83)
        if existing:
            cluster, _offset, entry = existing
            if entry[11] & 0x10 == 0:
                raise RuntimeError(f"{name} exists but is not a directory")
            return entry_cluster(entry)

        new_cluster = self.allocate_clusters(1)[0]
        self.write_cluster(new_cluster, dot_entries(new_cluster, parent_cluster))
        cluster, offset = self.find_free_entry(parent_cluster)
        self.write_entry(cluster, offset, make_entry(name83, 0x10, new_cluster, 0))
        return new_cluster

    def write_file(self, path, data):
        parts = [part for part in path.replace("\\", "/").split("/") if part]
        if not parts:
            raise RuntimeError("Destination path is empty")

        dir_cluster = self.root_cluster
        for part in parts[:-1]:
            dir_cluster = self.ensure_dir(dir_cluster, part)

        file_name = parts[-1]
        short_name = to_short_alias(file_name)
        existing = self.find_entry(dir_cluster, short_name)
        if existing:
            cluster, offset, entry = existing
            if entry[11] & 0x10:
                raise RuntimeError(f"{parts[-1]} exists but is a directory")
            self.free_chain(entry_cluster(entry))
            if is_83_name(file_name):
                entry_slots = [(cluster, offset)]
            else:
                self.delete_entry_and_lfns(dir_cluster, cluster, offset)
                entry_slots = self.find_free_entries(dir_cluster, lfn_entry_count(file_name) + 1)
        else:
            lfn_slots = 0 if is_83_name(file_name) else lfn_entry_count(file_name)
            entry_slots = self.find_free_entries(dir_cluster, lfn_slots + 1)

        cluster_count = max(1, math.ceil(len(data) / self.cluster_size))
        file_clusters = self.allocate_clusters(cluster_count)
        for index, cluster_id in enumerate(file_clusters):
            start = index * self.cluster_size
            self.write_cluster(cluster_id, data[start : start + self.cluster_size])

        entries = [] if is_83_name(file_name) else make_lfn_entries(file_name, short_name)
        entries.append(make_entry(short_name, 0x20, file_clusters[0], len(data)))
        for (cluster, offset), entry in zip(entry_slots, entries):
            self.write_entry(cluster, offset, entry)


def to_83(name):
    base, dot, ext = name.partition(".")
    base = base.upper()
    ext = ext.upper() if dot else ""
    allowed = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789$%'-_@~`!(){}^#&"
    if not base or len(base) > 8 or len(ext) > 3:
        raise RuntimeError(f"{name} is not an 8.3 FAT filename")
    if any(ch not in allowed for ch in base + ext):
        raise RuntimeError(f"{name} contains unsupported FAT filename characters")
    return base.encode("ascii").ljust(8, b" ") + ext.encode("ascii").ljust(3, b" ")


def to_short_alias(name):
    try:
        return to_83(name)
    except RuntimeError:
        pass

    base, dot, ext = name.partition(".")
    allowed = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789$%'-_@~`!(){}^#&"
    base = "".join(ch for ch in base.upper() if ch in allowed)
    ext = "".join(ch for ch in ext.upper() if ch in allowed)
    if not base:
        base = "FILE"
    if dot:
        return (base[:6] + "~1").encode("ascii").ljust(8, b" ") + ext[:3].encode("ascii").ljust(3, b" ")
    return base[:8].encode("ascii").ljust(8, b" ") + b"   "


def is_83_name(name):
    try:
        to_83(name)
        return True
    except RuntimeError:
        return False


def lfn_entry_count(name):
    return math.ceil((len(name) + 1) / 13)


def short_name_checksum(name83):
    checksum = 0
    for byte in name83:
        checksum = (((checksum & 1) << 7) + (checksum >> 1) + byte) & 0xFF
    return checksum


def make_lfn_entries(name, short_name):
    chars = [ord(ch) for ch in name]
    chars.append(0)
    while len(chars) % 13 != 0:
        chars.append(0xFFFF)

    checksum = short_name_checksum(short_name)
    chunks = [chars[index : index + 13] for index in range(0, len(chars), 13)]
    entries = []
    for index, chunk in reversed(list(enumerate(chunks, start=1))):
        entry = bytearray(32)
        entry[0] = index | (0x40 if index == len(chunks) else 0)
        entry[11] = 0x0F
        entry[13] = checksum
        for value, offset in zip(chunk[0:5], [1, 3, 5, 7, 9]):
            struct.pack_into("<H", entry, offset, value)
        for value, offset in zip(chunk[5:11], [14, 16, 18, 20, 22, 24]):
            struct.pack_into("<H", entry, offset, value)
        for value, offset in zip(chunk[11:13], [28, 30]):
            struct.pack_into("<H", entry, offset, value)
        entries.append(bytes(entry))
    return entries


def entry_cluster(entry):
    high = struct.unpack_from("<H", entry, 20)[0]
    low = struct.unpack_from("<H", entry, 26)[0]
    return (high << 16) | low


def make_entry(name83, attr, start_cluster, size):
    entry = bytearray(32)
    entry[:11] = name83
    entry[11] = attr
    struct.pack_into("<H", entry, 20, (start_cluster >> 16) & 0xFFFF)
    struct.pack_into("<H", entry, 26, start_cluster & 0xFFFF)
    struct.pack_into("<I", entry, 28, size)
    return bytes(entry)


def dot_entries(cluster, parent):
    entries = bytearray(64)
    entries[:32] = make_entry(b".          ", 0x10, cluster, 0)
    entries[32:64] = make_entry(b"..         ", 0x10, parent, 0)
    return bytes(entries)


def main():
    parser = argparse.ArgumentParser(description="Write a small file into a FAT32 image.")
    parser.add_argument("image")
    parser.add_argument("destination")
    parser.add_argument("source")
    args = parser.parse_args()

    with open(args.source, "rb") as source_file:
        data = source_file.read()

    image = Fat32Image(args.image)
    try:
        image.write_file(args.destination, data)
    finally:
        image.close()


if __name__ == "__main__":
    main()
