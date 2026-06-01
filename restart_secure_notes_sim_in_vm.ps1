$ErrorActionPreference = "Stop"

$key = "$env:USERPROFILE\.ssh\foundation_vm_ed25519"
$remote = "foundation@127.0.0.1"

$sshBase = @(
    "-i", $key,
    "-p", "2222",
    "-o", "BatchMode=yes",
    "-o", "ConnectTimeout=5",
    "-o", "ServerAliveInterval=5",
    "-o", "ServerAliveCountMax=2",
    "-o", "StrictHostKeyChecking=no",
    "-o", "UserKnownHostsFile=NUL"
)

Write-Host "Restarting Secure Notes simulator..."
& ssh @sshBase $remote "chmod +x ~/restart_secure_notes_simulator_now.sh ~/vm_run_secure_notes_sim_visible.sh && timeout 900s bash ~/restart_secure_notes_simulator_now.sh"
if ($LASTEXITCODE -ne 0) {
    throw "Secure Notes simulator restart failed with exit code $LASTEXITCODE"
}

Write-Host "Secure Notes simulator restart command complete."
