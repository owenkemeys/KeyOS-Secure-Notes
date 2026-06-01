#!/usr/bin/env bash
set -euo pipefail

# Codex workflow note:
# Run this after the user has approved the build/restart/test loop, unless the
# prompt explicitly asked to do the whole loop. This launcher leaves Weston
# large/lowered for review. It keeps the successful build output warm; heavier
# target/Nix cleanup belongs just before a new build, not after launch.

log_file="$HOME/secure-notes-restart.log"

disk_available_kb() {
  df -Pk "$HOME" | awk 'NR==2 {print $4}'
}

post_launch_health_report() {
  echo "[$(date --iso-8601=seconds)] Post-launch health report started"
  local project_target="$HOME/secure-notes/target"
  local resolved_target
  resolved_target="$(realpath -m "$project_target")"

  case "$resolved_target" in
    "$HOME/secure-notes/target")
      if [ -d "$resolved_target" ]; then
        du -sh "$resolved_target" 2>/dev/null | sed "s/^/[$(date --iso-8601=seconds)] Keeping warm fork target: /"
      else
        echo "[$(date --iso-8601=seconds)] Secure Notes target is absent; next launch may rebuild"
      fi
      ;;
    *)
      echo "[$(date --iso-8601=seconds)] Refusing to inspect unexpected target path: $resolved_target"
      ;;
  esac

  local available_kb
  available_kb="$(disk_available_kb)"
  if [ "${available_kb:-0}" -lt 10485760 ]; then
    echo "[$(date --iso-8601=seconds)] Warning: free space below 10GiB; run pre-build cleanup before the next build"
  fi

  df -h / | sed "s/^/[$(date --iso-8601=seconds)] /"
  free -h | sed "s/^/[$(date --iso-8601=seconds)] /"
  echo "[$(date --iso-8601=seconds)] Post-launch health report completed"
}

ensure_x_display() {
  if DISPLAY=:0 xdpyinfo >/dev/null 2>&1; then
    echo "[$(date --iso-8601=seconds)] X display :0 is available"
    return 0
  fi

  echo "[$(date --iso-8601=seconds)] X display :0 is missing; starting Openbox session"
  pkill -f "Xorg :0" 2>/dev/null || true
  pkill -f "openbox-session" 2>/dev/null || true
  nohup startx /usr/bin/openbox-session -- :0 > "$HOME/xsession.log" 2>&1 &

  for _ in {1..60}; do
    if DISPLAY=:0 xdpyinfo >/dev/null 2>&1; then
      echo "[$(date --iso-8601=seconds)] X display :0 started"
      return 0
    fi
    sleep 0.5
  done

  echo "[$(date --iso-8601=seconds)] Failed to start X display :0"
  tail -n 80 "$HOME/xsession.log" 2>/dev/null || true
  return 2
}

wait_for_wayland_socket() {
  local socket_path="/run/user/1000/wayland-1"
  for _ in {1..80}; do
    if [ -S "$socket_path" ]; then
      echo "[$(date --iso-8601=seconds)] Weston Wayland socket is ready"
      return 0
    fi
    if ! pgrep -f "weston --backend=x11-backend.so" >/dev/null 2>&1; then
      echo "[$(date --iso-8601=seconds)] Weston exited before creating $socket_path"
      tail -n 80 "$HOME/weston.log" 2>/dev/null || true
      return 3
    fi
    sleep 0.25
  done

  echo "[$(date --iso-8601=seconds)] Weston did not create $socket_path in time"
  tail -n 80 "$HOME/weston.log" 2>/dev/null || true
  return 3
}

{
  echo "[$(date --iso-8601=seconds)] Secure Notes restart requested"

  pkill -f "foundation sim" 2>/dev/null || true
  pkill -f "/keyos/simulator/" 2>/dev/null || true
  pkill -f "target/apps/secure-notes/app.elf" 2>/dev/null || true
  pkill -f "weston --backend=x11-backend.so" 2>/dev/null || true
  sleep 1

  mkdir -p "$HOME/.config"
  cat > "$HOME/.config/weston.ini" <<'CFG'
[core]
idle-time=0

[shell]
locking=false
background-color=0xff3a3a3a
CFG

  ensure_x_display

  DISPLAY=:0 openbox --reconfigure >/dev/null 2>&1 || true

  DISPLAY=:0 XDG_RUNTIME_DIR=/run/user/1000 nohup weston \
      --backend=x11-backend.so \
      --socket=wayland-1 \
      --width=1600 \
      --height=900 \
      --idle-time=0 \
      > "$HOME/weston.log" 2>&1 &

  wait_for_wayland_socket

  if command -v xdotool >/dev/null 2>&1; then
    weston_window=""
    for _ in {1..40}; do
      weston_window="$(DISPLAY=:0 xdotool search --onlyvisible --class "Weston Compositor" 2>/dev/null | head -n 1 || true)"
      [ -n "$weston_window" ] && break
      sleep 0.25
    done

    if [ -n "$weston_window" ]; then
      DISPLAY=:0 xdotool windowactivate "$weston_window" >/dev/null 2>&1 || true
      sleep 0.2
      DISPLAY=:0 xdotool key --clearmodifiers Alt+F10 >/dev/null 2>&1 || true
      sleep 0.5
      DISPLAY=:0 xdotool windowmove "$weston_window" 0 38 windowsize "$weston_window" 1920 1061 >/dev/null 2>&1 || true
      DISPLAY=:0 xdotool windowlower "$weston_window" >/dev/null 2>&1 || true
    fi
  fi

  : > "$HOME/foundation-secure-notes-sim.log"
  source /etc/profile.d/nix.sh
  cd "$HOME/.foundation/sdk/current"
  nohup nix develop --command "$HOME/vm_run_secure_notes_sim_visible.sh" \
      > "$HOME/foundation-secure-notes-sim.log" 2>&1 &

  launched=0
  for _ in {1..600}; do
    if pgrep -af "/target/apps/secure-notes/app.elf" | grep -v -E "pgrep|grep|restart_secure_notes" >/dev/null 2>&1; then
      echo "[$(date --iso-8601=seconds)] Secure Notes app detected"
      launched=1
      break
    fi
    sleep 1
  done

  if [ "$launched" -eq 1 ]; then
    post_launch_health_report
  else
    echo "[$(date --iso-8601=seconds)] Secure Notes app was not detected; skipping post-launch health report"
  fi

  if command -v xdotool >/dev/null 2>&1; then
    weston_window="$(DISPLAY=:0 xdotool search --onlyvisible --class "Weston Compositor" 2>/dev/null | head -n 1 || true)"
    if [ -n "$weston_window" ]; then
      DISPLAY=:0 xdotool windowmove "$weston_window" 0 38 windowsize "$weston_window" 1920 1061 windowlower "$weston_window" >/dev/null 2>&1 || true
    fi
  fi

  echo "[$(date --iso-8601=seconds)] Secure Notes restart launcher completed"
} >> "$log_file" 2>&1
