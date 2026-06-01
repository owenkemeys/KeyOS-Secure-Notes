$ErrorActionPreference = "SilentlyContinue"

$workingRoot = Split-Path -Parent $PSCommandPath
$destination = Join-Path $workingRoot "sim_screenshots\from-simulator"
$sshKey = Join-Path $env:USERPROFILE ".ssh\foundation_vm_ed25519"
$remoteDir = "/home/foundation/secure-notes/screenshots"
$remote = "foundation@127.0.0.1"
$statePath = Join-Path $destination ".sync-state.json"
$keepers = Join-Path $destination "keepers"

New-Item -ItemType Directory -Force -Path $destination | Out-Null
New-Item -ItemType Directory -Force -Path $keepers | Out-Null

$state = @{}
if (Test-Path -LiteralPath $statePath) {
    $rawState = Get-Content -Raw -LiteralPath $statePath
    if (-not [string]::IsNullOrWhiteSpace($rawState)) {
        $parsed = $rawState | ConvertFrom-Json
        foreach ($property in $parsed.PSObject.Properties) {
            $state[$property.Name] = [int64]$property.Value
        }
    }
}

function Save-SyncState {
    $state |
        ConvertTo-Json -Compress |
        Set-Content -LiteralPath $statePath -Encoding ASCII
    & attrib +h $statePath 2>$null
}

while ($true) {
    $files = & ssh `
        -i $sshKey `
        -p 2222 `
        -o StrictHostKeyChecking=no `
        -o UserKnownHostsFile=NUL `
        $remote `
        "find '$remoteDir' -maxdepth 1 -type f \( -iname '*.png' -o -iname '*.gif' -o -iname '*.mp4' -o -iname '*.webm' \) -printf '%f`t%s`n'"

    foreach ($line in $files) {
        if ([string]::IsNullOrWhiteSpace($line)) {
            continue
        }

        $parts = $line -split "`t", 2
        if ($parts.Count -ne 2) {
            continue
        }

        $name = $parts[0]
        $remoteSize = [int64]$parts[1]
        $localPath = Join-Path $destination $name
        $localExists = Test-Path -LiteralPath $localPath
        $knownSize = $null
        if ($state.ContainsKey($name)) {
            $knownSize = $state[$name]
        }

        if (-not $localExists -and $knownSize -eq $remoteSize) {
            continue
        }

        $needsCopy = -not $localExists
        if ($localExists) {
            $needsCopy = (Get-Item -LiteralPath $localPath).Length -ne $remoteSize
        }

        if ($needsCopy) {
            & scp `
                -i $sshKey `
                -P 2222 `
                -o StrictHostKeyChecking=no `
                -o UserKnownHostsFile=NUL `
                -p `
                "$remote`:$remoteDir/$name" `
                $destination | Out-Null
        }

        if (Test-Path -LiteralPath $localPath) {
            $state[$name] = (Get-Item -LiteralPath $localPath).Length
            Save-SyncState
        }
    }

    Start-Sleep -Seconds 3
}
