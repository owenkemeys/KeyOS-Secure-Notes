$ErrorActionPreference = "Stop"

$key = "$env:USERPROFILE\.ssh\foundation_vm_ed25519"
$remote = "foundation@127.0.0.1"

$sshBase = @(
    "-i", $key,
    "-p", "2222",
    "-o", "BatchMode=yes",
    "-o", "ConnectTimeout=5",
    "-o", "StrictHostKeyChecking=no",
    "-o", "UserKnownHostsFile=NUL"
)

& ssh @sshBase $remote "tail -n 100 ~/secure-notes-restart.log 2>/dev/null || true; echo '---simlog---'; tail -n 100 ~/foundation-secure-notes-sim.log 2>/dev/null || true; echo '---process---'; pgrep -af 'secure-notes/app.elf|foundation sim|weston --backend' || true"
