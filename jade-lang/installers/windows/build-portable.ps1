# Build a portable Windows zip: jade.exe + jade.ico (no installer).
# Usage: .\build-portable.ps1 [-Arch x86_64|i686|aarch64]

param([string]$Arch = "x86_64")

$ErrorActionPreference = "Stop"
$version = "1.0.0"
$zipName = "jade-$version-windows-$Arch-portable.zip"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$jadeLang = Split-Path -Parent (Split-Path -Parent $scriptDir)
$repoRoot = Split-Path -Parent $jadeLang
if (-not (Test-Path (Join-Path $jadeLang "Cargo.toml"))) {
    $jadeLang = $scriptDir
    while (-not (Test-Path (Join-Path $jadeLang "Cargo.toml"))) {
        $jadeLang = Split-Path -Parent $jadeLang
        if (-not $jadeLang) { throw "Cannot find jade-lang (Cargo.toml)" }
    }
    $repoRoot = Split-Path -Parent $jadeLang
}

$targetMap = @{ "x86_64" = "x86_64-pc-windows-msvc"; "i686" = "i686-pc-windows-msvc"; "aarch64" = "aarch64-pc-windows-msvc" }
$rustTarget = $targetMap[$Arch]; if (-not $rustTarget) { $rustTarget = "x86_64-pc-windows-msvc"; $Arch = "x86_64" }
$binaryPath = if ($Arch -eq "x86_64") { Join-Path $jadeLang "target\release\jade.exe" } else { Join-Path $jadeLang "target\$rustTarget\release\jade.exe" }
$iconPath = Join-Path $scriptDir "icon\jade.ico"
$outDir = Join-Path $repoRoot "dist\installers\windows"
$stagingDir = Join-Path $env:TEMP "jade-portable-staging"
$zipPath = Join-Path $outDir $zipName

if (-not (Test-Path $binaryPath)) {
    Write-Host "Build the binary first: cd jade-lang; cargo build --release"
    exit 1
}

New-Item -ItemType Directory -Force -Path $outDir | Out-Null
if (Test-Path $stagingDir) { Remove-Item -Recurse -Force $stagingDir }
New-Item -ItemType Directory -Force -Path $stagingDir | Out-Null

Copy-Item -Path $binaryPath -Destination (Join-Path $stagingDir "jade.exe") -Force
if (Test-Path $iconPath) {
    Copy-Item -Path $iconPath -Destination (Join-Path $stagingDir "jade.ico") -Force
}

Push-Location $stagingDir
Compress-Archive -Path "jade.exe", "jade.ico" -DestinationPath $zipPath -Force
Pop-Location
Remove-Item -Recurse -Force $stagingDir

Write-Host "Created: $zipPath"
