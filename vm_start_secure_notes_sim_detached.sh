#!/usr/bin/env bash
set -euo pipefail

pkill -f "foundation sim" 2>/dev/null || true
pkill -f "/keyos/simulator/" 2>/dev/null || true
pkill -f "target/apps/qr-intent-inspector/app.elf" 2>/dev/null || true
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

DISPLAY=:0 XDG_RUNTIME_DIR=/run/user/1000 nohup weston \
    --backend=x11-backend.so \
    --socket=wayland-1 \
    --width=1600 \
    --height=900 \
    --idle-time=0 \
    > "$HOME/weston.log" 2>&1 &
sleep 2

# Weston is the host Openbox window. Maximize it like the titlebar button
# instead of using `weston --fullscreen`, which can leave a stale small
# viewport floating in a large black VM area.
if command -v xdotool >/dev/null 2>&1; then
    weston_window=""
    for _ in {1..20}; do
        weston_window="$(DISPLAY=:0 xdotool search --onlyvisible --class "Weston Compositor" 2>/dev/null | head -n 1 || true)"
        [ -n "$weston_window" ] && break
        sleep 0.25
    done

    if [ -n "$weston_window" ]; then
        eval "$(DISPLAY=:0 xdotool getwindowgeometry --shell "$weston_window")"
        title_x=$((X + WIDTH / 2))
        title_y=$((Y - 30))
        if [ "$title_y" -lt 10 ]; then
            title_y=10
        fi
        DISPLAY=:0 xdotool mousemove "$title_x" "$title_y" click --repeat 2 --delay 100 1
        sleep 1
    fi
fi

rm -f "$HOME/foundation-secure-notes-sim.log"
source /etc/profile.d/nix.sh
cd "$HOME/.foundation/sdk/current"

nohup nix develop --command "$HOME/vm_run_secure_notes_sim_visible.sh" \
    > "$HOME/foundation-secure-notes-sim.log" 2>&1 &
echo "$!" > "$HOME/foundation-secure-notes-sim.pid"
