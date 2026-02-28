# Run a bootstrap script using the repo-built jade.exe (no PATH needed).
#
# Usage (use .\ when in repo root "Jade", use ..\ only when cd'd into jade-lang):
#   .\bootstrap\run.ps1 stage0_hello.jdl    # from repo root
#   ..\bootstrap\run.ps1 stage0_hello.jdl  # from inside jade-lang folder

$ErrorActionPreference = "Stop"
$repoRoot = Split-Path -Parent $PSScriptRoot
$script = $args[0]
if (-not $script) {
    Write-Host "Usage: .\bootstrap\run.ps1 <script.jdl>"
    Write-Host "Example: .\bootstrap\run.ps1 stage0_hello.jdl"
    Write-Host "From repo root use: .\bootstrap\run.ps1 stage0_hello.jdl"
    exit 1
}
$scriptPath = Join-Path $repoRoot "bootstrap\$script"
if (-not (Test-Path $scriptPath)) {
    Write-Host "Script not found: $scriptPath"
    exit 1
}

# Prefer jade.exe next to repo, then in current dir (e.g. when in jade-lang)
$jade = Join-Path $repoRoot "jade-lang\target\release\jade.exe"
if (-not (Test-Path $jade)) {
    $jade = Join-Path (Get-Location) "target\release\jade.exe"
}
if (-not (Test-Path $jade)) {
    Write-Host "Jade not found. Tried:"
    Write-Host "  $([System.IO.Path]::Combine($repoRoot, 'jade-lang\target\release\jade.exe'))"
    Write-Host "  $([System.IO.Path]::Combine((Get-Location), 'target\release\jade.exe'))"
    Write-Host "Build from repo root: cd jade-lang; cargo build --release"
    exit 1
}

$remaining = if ($args.Length -gt 1) { $args[1..($args.Length - 1)] } else { @() }
Push-Location $repoRoot
try {
    & $jade $scriptPath @remaining
} finally {
    Pop-Location
}
