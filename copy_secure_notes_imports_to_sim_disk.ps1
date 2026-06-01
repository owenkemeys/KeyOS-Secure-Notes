param(
    [switch]$AllowStopSimulator,
    [switch]$ResetAppDatabase
)

$ErrorActionPreference = "Stop"

if (-not $AllowStopSimulator) {
    throw "Refusing to stop the simulator without -AllowStopSimulator. Ask the user first unless this is part of a build/run request."
}

$root = Split-Path -Parent $PSCommandPath
$key = "$env:USERPROFILE\.ssh\foundation_vm_ed25519"
$remote = "foundation@127.0.0.1"
$port = "2222"
$remoteDir = "/home/foundation/sim-import-copy"

$sshBase = @(
    "-i", $key,
    "-p", $port,
    "-o", "BatchMode=yes",
    "-o", "ConnectTimeout=5",
    "-o", "ServerAliveInterval=5",
    "-o", "ServerAliveCountMax=2",
    "-o", "StrictHostKeyChecking=no",
    "-o", "UserKnownHostsFile=NUL"
)

$scpBase = @(
    "-i", $key,
    "-P", $port,
    "-o", "BatchMode=yes",
    "-o", "ConnectTimeout=5",
    "-o", "StrictHostKeyChecking=no",
    "-o", "UserKnownHostsFile=NUL"
)

$files = @(
    "$root\tools\write_fat32_file.py",
    "$root\copy_secure_notes_imports_to_sim_disk.sh",
    "$root\test-imports\bitwarden-stress-mixed-1.json",
    "$root\test-imports\bitwarden-stress-mixed-2.json",
    "$root\test-imports\bitwarden-stress-20.json",
    "$root\test-imports\secure-notes-sim-seed.json"
)

foreach ($file in $files) {
    if (-not (Test-Path -LiteralPath $file)) {
        throw "Missing required file: $file"
    }
}

Write-Host "Checking SSH connectivity..."
& ssh @sshBase $remote "printf 'vm-ok\n'"
if ($LASTEXITCODE -ne 0) {
    throw "SSH connectivity check failed"
}

Write-Host "Preparing remote staging directory..."
& ssh @sshBase $remote "rm -rf '$remoteDir' && mkdir -p '$remoteDir'"
if ($LASTEXITCODE -ne 0) {
    throw "Remote staging directory setup failed"
}

Write-Host "Copying import files and FAT writer to VM..."
& scp @scpBase $files "$remote`:$remoteDir/"
if ($LASTEXITCODE -ne 0) {
    throw "SCP copy failed"
}

Write-Host "Writing files into simulator disk image with a 45 second timeout..."
$resetValue = if ($ResetAppDatabase) { "1" } else { "0" }
& ssh @sshBase $remote "chmod +x '$remoteDir/copy_secure_notes_imports_to_sim_disk.sh' && ALLOW_STOP_SIM=1 RESET_SECURE_NOTES_DB=$resetValue timeout 45s bash '$remoteDir/copy_secure_notes_imports_to_sim_disk.sh'"
if ($LASTEXITCODE -ne 0) {
    throw "Simulator disk import failed or timed out"
}

Write-Host "Simulator import files are staged in the simulated internal storage."
