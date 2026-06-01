$ErrorActionPreference = "Stop"

$workingRoot = Split-Path -Parent $PSCommandPath
$script = Join-Path $workingRoot "sync_secure_notes_sim_captures.ps1"
$destination = Join-Path $workingRoot "sim_screenshots\from-simulator"

New-Item -ItemType Directory -Force -Path $destination | Out-Null

$existing = Get-CimInstance Win32_Process |
    Where-Object { $_.CommandLine -like "*sync_secure_notes_sim_captures.ps1*" }

if (-not $existing) {
    Start-Process powershell.exe -WindowStyle Hidden -ArgumentList "-NoProfile -ExecutionPolicy Bypass -File `"$script`""
}

Write-Output "Simulator captures are syncing to: $destination"
