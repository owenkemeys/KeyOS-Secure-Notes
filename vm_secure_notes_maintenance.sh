#!/usr/bin/env bash
set -euo pipefail

app_root="/home/foundation/secure-notes"
draft_root="/home/foundation/secure-notes"
capture_dir="$app_root/screenshots"
bundle_root="$draft_root/bundles"
desktop_dir="$HOME/Desktop"
bin_dir="$HOME/bin"

mkdir -p "$capture_dir" "$bundle_root" "$desktop_dir" "$bin_dir"

if [ -d "$app_root/target/keyos/secure-notes" ]; then
  rm -rf "$bundle_root/secure-notes-latest"
  cp -a "$app_root/target/keyos/secure-notes" "$bundle_root/secure-notes-latest"
fi

safe_rm_dir() {
  local path="$1"
  [ -d "$path" ] || return 0

  local resolved
  resolved="$(realpath -m "$path")"
  case "$resolved" in
    /home/foundation/*/target|/home/foundation/*/*/target)
      rm -rf -- "$resolved"
      ;;
    *)
      echo "Refusing to remove unexpected path: $resolved" >&2
      return 2
      ;;
  esac
}

if [ "${1:-}" = "--pre-build" ]; then
  available_kb="$(df -Pk "$HOME" | awk 'NR==2 {print $4}')"
  if [ "${available_kb:-0}" -lt 31457280 ]; then
    echo "Free space below 30GiB; cleaning known rebuildable targets before build."
    safe_rm_dir "$app_root/target"
    safe_rm_dir "/home/foundation/qr-inspector/qr-intent-inspector/target"
  else
    echo "Pre-build target cleanup skipped; free space is sufficient."
  fi
else
  echo "Skipping target removal. Pass --pre-build to clean rebuildable targets under disk pressure."
fi

ln -sfn "$capture_dir" "$desktop_dir/Secure Notes Simulator Captures"

cat > "$bin_dir/open-secure-notes-captures.sh" <<'SCRIPT'
#!/usr/bin/env bash
set -euo pipefail
capture_dir="/home/foundation/secure-notes/screenshots"
mkdir -p "$capture_dir"
if command -v xdg-open >/dev/null 2>&1; then
  xdg-open "$capture_dir" >/tmp/secure-notes-captures-open.log 2>&1 || true
fi
if command -v xterm >/dev/null 2>&1; then
  xterm -T "Secure Notes Simulator Captures" -geometry 120x32 -e "watch -n 1 'printf \"Secure Notes simulator captures\\n%s\\n\\n\" \"$capture_dir\"; ls -lhtr \"$capture_dir\" | tail -40'" &
fi
SCRIPT
chmod +x "$bin_dir/open-secure-notes-captures.sh"

cat > "$desktop_dir/Open Secure Notes Captures.desktop" <<'DESKTOP'
[Desktop Entry]
Type=Application
Name=Open Secure Notes Captures
Comment=Show the folder where Passport simulator screenshots are saved
Exec=/home/foundation/bin/open-secure-notes-captures.sh
Terminal=false
Categories=Utility;
DESKTOP
chmod +x "$desktop_dir/Open Secure Notes Captures.desktop"

if [ "${1:-}" = "--pre-build" ]; then
  available_kb="$(df -Pk "$HOME" | awk 'NR==2 {print $4}')"
  if [ "${available_kb:-0}" -lt 31457280 ]; then
    nix-collect-garbage -d >/tmp/secure-notes-cleanup-nix.log 2>&1 || true
    sudo apt-get clean || true
  else
    echo "Skipping Nix GC; free space is sufficient after target check."
  fi
fi

echo "Capture folder: $capture_dir"
echo "Desktop shortcut: $desktop_dir/Secure Notes Simulator Captures"
df -h /
du -sh "$draft_root" "$capture_dir" "$bundle_root" 2>/dev/null || true
