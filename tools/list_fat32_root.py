#!/usr/bin/env python3
import struct
import sys

EOC = 0x0FFFFFFF


class Fat32Image:
    def __init__(self, image_path):
        self.image = open(image_path, "rb")
        boot = self.image.read(512)
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

    def read_at(self, offset, size):
        self.image.seek(offset)
        return self.image.read(size)

    def cluster_offset(self, cluster):
        return self.data_offset + (cluster - 2) * self.cluster_size

    def read_cluster(self, cluster):
        return self.read_at(self.cluster_offset(cluster), self.cluster_size)

    def fat_entry(self, cluster):
        raw = self.read_at(self.fat_offset + cluster * 4, 4)
        return struct.unpack("<I", raw)[0] & 0x0FFFFFFF

    def cluster_chain(self, start):
        cluster = start
        while 2 <= cluster < EOC:
            yield cluster
            next_cluster = self.fat_entry(cluster)
            if next_cluster >= 0x0FFFFFF8:
                return
            cluster = next_cluster


def decode_short(entry):
    base = entry[:8].decode("ascii", "replace").rstrip()
    ext = entry[8:11].decode("ascii", "replace").rstrip()
    return f"{base}.{ext}" if ext else base


def decode_lfn(entry):
    chars = []
    for offset in [1, 3, 5, 7, 9, 14, 16, 18, 20, 22, 24, 28, 30]:
        value = struct.unpack_from("<H", entry, offset)[0]
        if value in (0, 0xFFFF):
            continue
        chars.append(chr(value))
    return "".join(chars)


def entry_cluster(entry):
    high = struct.unpack_from("<H", entry, 20)[0]
    low = struct.unpack_from("<H", entry, 26)[0]
    return (high << 16) | low


def list_dir(image, cluster, prefix=""):
    lfn_parts = []
    subdirs = []
    for cluster in image.cluster_chain(cluster):
        data = image.read_cluster(cluster)
        for offset in range(0, image.cluster_size, 32):
            entry = data[offset : offset + 32]
            if entry[0] == 0x00:
                break
            if entry[0] == 0xE5:
                lfn_parts = []
                continue
            attr = entry[11]
            if attr == 0x0F:
                lfn_parts.insert(0, decode_lfn(entry))
                continue
            long_name = "".join(lfn_parts)
            lfn_parts = []
            flags = []
            if attr & 0x10:
                flags.append("DIR")
            if attr & 0x20:
                flags.append("FILE")
            size = struct.unpack_from("<I", entry, 28)[0]
            short = decode_short(entry)
            display_name = long_name or short
            print(f"{prefix}{short:14} {long_name:32} {','.join(flags):8} {size}")
            if attr & 0x10 and short not in (".", ".."):
                subdirs.append((entry_cluster(entry), f"{prefix}{display_name}/"))
    for sub_cluster, sub_prefix in subdirs:
        list_dir(image, sub_cluster, sub_prefix)


def main():
    image = Fat32Image(sys.argv[1])
    list_dir(image, image.root_cluster)


if __name__ == "__main__":
    main()
