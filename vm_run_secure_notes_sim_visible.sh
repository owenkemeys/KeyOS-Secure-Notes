#!/usr/bin/env bash
set -euo pipefail

unset DISPLAY
SIM_LIBS="$HOME/sim-libs"
export XDG_RUNTIME_DIR=/run/user/1000
export WAYLAND_DISPLAY=wayland-1
export WINIT_UNIX_BACKEND=wayland
export RUST_BACKTRACE=1
export XKB_CONFIG_ROOT="$SIM_LIBS/xkeyboard-config/share/X11/xkb"
export FONTCONFIG_FILE="$SIM_LIBS/fontconfig/etc/fonts/fonts.conf"
export LD_LIBRARY_PATH="$SIM_LIBS/wayland/lib:$SIM_LIBS/libxkbcommon/lib:$SIM_LIBS/libffi/lib:$SIM_LIBS/fontconfig-lib/lib:${LD_LIBRARY_PATH:-}"
export PATH="$HOME/.foundation/sdk/current/bin:$HOME/.foundation/sdk/bin:$PATH"

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

available_kb="$(disk_available_kb)"
if [ "${available_kb:-0}" -lt 31457280 ]; then
  echo "Free space below 30GiB before simulator build/run; cleaning known rebuildable target."
  safe_rm_dir "$HOME/secure-notes/target"
  available_kb="$(disk_available_kb)"
  if [ "${available_kb:-0}" -lt 31457280 ]; then
    echo "Free space still below 30GiB; running Nix garbage collection and apt clean."
    source /etc/profile.d/nix.sh
    nix-collect-garbage -d
    sudo apt-get clean || true
  fi
else
  echo "Pre-sim cleanup skipped; preserving current build target for handoff."
fi

cd "$HOME/secure-notes"
exec "$HOME/.foundation/sdk/current/bin/foundation" sim
