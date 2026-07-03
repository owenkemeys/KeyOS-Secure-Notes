param(
    [string]$Root = ""
)

$ErrorActionPreference = "Stop"
if ([string]::IsNullOrWhiteSpace($Root)) {
    $Root = Split-Path -Parent $PSCommandPath
}

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

$scpBase = @(
    "-i", $key,
    "-P", "2222",
    "-o", "BatchMode=yes",
    "-o", "ConnectTimeout=5",
    "-o", "StrictHostKeyChecking=no",
    "-o", "UserKnownHostsFile=NUL"
)

$required = @(
    "$Root\secure-notes\ui\app.slint",
    "$Root\secure-notes\src\main.rs",
    "$Root\secure-notes\Cargo.toml",
    "$Root\secure-notes-core\src\lib.rs",
    "$Root\secure-notes-core\Cargo.toml",
    "$Root\secure-notes\resources\keyos-background-light.png",
    "$Root\secure-notes\resources\keyos-background-dark.png",
    "$Root\secure-notes\resources\background-sources.json",
    "$Root\secure-notes\resources\icon.svg",
    "$Root\secure-notes\resources\icons\icon-sources.json",
    "$Root\vm_build_secure_notes.sh",
    "$Root\vm_run_secure_notes_sim_visible.sh",
    "$Root\restart_secure_notes_simulator_now.sh"
)

foreach ($path in $required) {
    if (-not (Test-Path -LiteralPath $path)) {
        throw "Missing required file: $path"
    }
}

Write-Host "Checking SSH connectivity..."
& ssh @sshBase $remote "printf 'vm-ok\n'"
if ($LASTEXITCODE -ne 0) {
    throw "SSH connectivity check failed"
}

Write-Host "Preparing remote Secure Notes directories..."
& ssh @sshBase $remote "mkdir -p /home/foundation/secure-notes/ui /home/foundation/secure-notes/src /home/foundation/secure-notes/resources/icons /home/foundation/secure-notes-core/src"
if ($LASTEXITCODE -ne 0) {
    throw "Remote mkdir failed"
}

Write-Host "Copying app UI source..."
& scp @scpBase "$Root\secure-notes\ui\app.slint" "$remote`:/home/foundation/secure-notes/ui/app.slint"
if ($LASTEXITCODE -ne 0) {
    throw "app.slint copy failed"
}

Write-Host "Copying Rust source..."
& scp @scpBase "$Root\secure-notes\src\main.rs" "$remote`:/home/foundation/secure-notes/src/main.rs"
if ($LASTEXITCODE -ne 0) {
    throw "main.rs copy failed"
}
& scp @scpBase "$Root\secure-notes\Cargo.toml" "$remote`:/home/foundation/secure-notes/Cargo.toml"
if ($LASTEXITCODE -ne 0) {
    throw "secure-notes Cargo.toml copy failed"
}
& scp @scpBase "$Root\secure-notes-core\src\lib.rs" "$remote`:/home/foundation/secure-notes-core/src/lib.rs"
if ($LASTEXITCODE -ne 0) {
    throw "secure-notes-core lib.rs copy failed"
}
& scp @scpBase "$Root\secure-notes-core\Cargo.toml" "$remote`:/home/foundation/secure-notes-core/Cargo.toml"
if ($LASTEXITCODE -ne 0) {
    throw "secure-notes-core Cargo.toml copy failed"
}

Write-Host "Copying KeyOS background and app icon assets..."
& scp @scpBase "$Root\secure-notes\resources\keyos-background-light.png" "$Root\secure-notes\resources\keyos-background-dark.png" "$Root\secure-notes\resources\background-sources.json" "$Root\secure-notes\resources\icon.svg" "$remote`:/home/foundation/secure-notes/resources/"
if ($LASTEXITCODE -ne 0) {
    throw "Background/app icon asset copy failed"
}

Write-Host "Copying SVG icon assets and manifest..."
& scp @scpBase "$Root\secure-notes\resources\icons\*.svg" "$Root\secure-notes\resources\icons\icon-sources.json" "$remote`:/home/foundation/secure-notes/resources/icons/"
if ($LASTEXITCODE -ne 0) {
    throw "Icon asset copy failed"
}

Write-Host "Copying VM helper scripts..."
& scp @scpBase "$Root\vm_build_secure_notes.sh" "$Root\vm_run_secure_notes_sim_visible.sh" "$Root\restart_secure_notes_simulator_now.sh" "$remote`:/home/foundation/"
if ($LASTEXITCODE -ne 0) {
    throw "VM helper script copy failed"
}

Write-Host "Secure Notes source copied to VM."
