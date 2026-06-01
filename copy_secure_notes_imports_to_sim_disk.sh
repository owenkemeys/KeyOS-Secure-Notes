#!/usr/bin/env bash
set -euo pipefail

work_dir="/home/foundation/sim-import-copy"
disk_image="/home/foundation/.foundation/sdk/current/lib/keyos/simulator/xous/kernel/disk.dat"
fat_writer="$work_dir/write_fat32_file.py"

log() {
  printf '[%s] %s\n' "$(date --iso-8601=seconds)" "$*"
}

fail() {
  log "FAILED: $*"
  exit 1
}

trap 'fail "line $LINENO while preparing simulator import files"' ERR

log "Checking simulator disk and input files"
[ -f "$disk_image" ] || fail "missing simulator disk image: $disk_image"
[ -f "$fat_writer" ] || fail "missing FAT writer: $fat_writer"

for name in bitwarden-stress-mixed-1.json bitwarden-stress-mixed-2.json bitwarden-stress-20.json; do
  [ -f "$work_dir/$name" ] || fail "missing input file: $work_dir/$name"
done
[ -f "$work_dir/secure-notes-sim-seed.json" ] || fail "missing seed database file: $work_dir/secure-notes-sim-seed.json"

if [ "${ALLOW_STOP_SIM:-0}" != "1" ]; then
  fail "refusing to stop the simulator without ALLOW_STOP_SIM=1; ask the user first unless this is part of a build/run request"
fi

log "Stopping simulator processes before editing disk image"
timeout 8s pkill -f "secure-notes/app.elf" 2>/dev/null || true
timeout 8s pkill -f "foundation sim" 2>/dev/null || true
timeout 8s pkill -f "/keyos/simulator/" 2>/dev/null || true
sleep 1

log "Writing Bitwarden stress files into simulator internal storage"
# The simulator's "Internal" picker shows the FAT `user/` directory. The
# original successful test used user/BWTEST.JSON, so keep this exact location.
# Use distinct short bases so the generated FAT short aliases do not collide.
python3 "$fat_writer" "$disk_image" "user/BW1.JSON" "$work_dir/bitwarden-stress-mixed-1.json"
python3 "$fat_writer" "$disk_image" "user/BW2.JSON" "$work_dir/bitwarden-stress-mixed-2.json"
python3 "$fat_writer" "$disk_image" "user/BW3.JSON" "$work_dir/bitwarden-stress-mixed-1.json"
python3 "$fat_writer" "$disk_image" "user/BW4.JSON" "$work_dir/bitwarden-stress-mixed-2.json"
python3 "$fat_writer" "$disk_image" "user/BW20.JSON" "$work_dir/bitwarden-stress-20.json"

if [ "${RESET_SECURE_NOTES_DB:-0}" = "1" ]; then
  log "Resetting Secure Notes app database to the compact review seed"
  python3 "$fat_writer" "$disk_image" "appdata/536563~2/secure_notes_v1.json" "$work_dir/secure-notes-sim-seed.json"
fi
sync

log "Done. Restart the simulator to make the files visible in the file picker. Look for BW1.JSON, BW2.JSON, BW3.JSON, BW4.JSON, and BW20.JSON next to BWTEST.JSON."
