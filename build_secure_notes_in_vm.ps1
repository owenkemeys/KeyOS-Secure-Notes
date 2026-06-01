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

Write-Host "Starting Secure Notes VM build..."
& ssh @sshBase $remote "chmod +x ~/vm_build_secure_notes.sh ~/vm_run_secure_notes_sim_visible.sh ~/restart_secure_notes_simulator_now.sh && timeout 2400s bash ~/vm_build_secure_notes.sh"
if ($LASTEXITCODE -ne 0) {
    throw "Secure Notes VM build failed with exit code $LASTEXITCODE"
}

Write-Host "Secure Notes VM build complete."
