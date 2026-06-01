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

& ssh @sshBase $remote "pkill -f 'foundation build' 2>/dev/null || true; pkill -f 'vm_build_secure_notes.sh' 2>/dev/null || true; pkill -f 'timeout 2400s bash' 2>/dev/null || true; printf 'stopped-build-processes\n'"
