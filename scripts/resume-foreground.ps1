param(
    [string]$Message = "continue",
    [string]$Exe = ".\target\release\codex-cua-resume-assist.exe"
)

$ErrorActionPreference = "Stop"

if (-not (Test-Path $Exe)) {
    cargo build --release | Out-Host
}

& $Exe --api --execute --terminal-send $Message
