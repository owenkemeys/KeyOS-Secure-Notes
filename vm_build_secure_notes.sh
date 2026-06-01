#!/usr/bin/env bash
set -euo pipefail

export PATH="$HOME/.foundation/sdk/current/bin:$HOME/.foundation/sdk/bin:$PATH"
app_root="$HOME/secure-notes"

disk_available_kb() {
  df -Pk "$HOME" | awk 'NR==2 {print $4}'
}

safe_rm_dir() {
  local path="$1"
  [ -d "$path" ] || return 0

  local resolved
  resolved="$(realpath -m "$path")"
  case "$resolved" in
    "$HOME"/secure-notes/target)
      du -sh "$resolved" 2>/dev/null | sed 's/^/Removing low-space rebuildable target: /'
      rm -rf -- "$resolved"
      ;;
    *)
      echo "Refusing to remove unexpected path: $resolved" >&2
      return 2
      ;;
  esac
}

pre_build_cleanup_if_needed() {
  echo "Starting a fresh Foundation build; cleaning known rebuildable app target first."
  safe_rm_dir "$app_root/target"

  local available_kb
  available_kb="$(disk_available_kb)"
  if [ "${available_kb:-0}" -ge 31457280 ]; then
    echo "Pre-build cleanup skipped; free space is sufficient."
    df -h /
    return 0
  fi

  echo "Free space below 30GiB before build after target cleanup; running Nix garbage collection and apt clean."
  available_kb="$(disk_available_kb)"
  if [ "${available_kb:-0}" -lt 31457280 ]; then
    source /etc/profile.d/nix.sh
    nix-collect-garbage -d
    sudo apt-get clean || true
  fi
  df -h /
}

pre_build_cleanup_if_needed
cd "$app_root"
pwd
ls -la app-config.toml Cargo.toml
which foundation || true
echo "FOUNDATION_SDK_ROOT=${FOUNDATION_SDK_ROOT:-}"
source /etc/profile.d/nix.sh
cd "$HOME/.foundation/sdk/current"
nix develop --command bash -lc 'cd "$HOME/secure-notes" && "$HOME/.foundation/sdk/current/bin/foundation" build'
