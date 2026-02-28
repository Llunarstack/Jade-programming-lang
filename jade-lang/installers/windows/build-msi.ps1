# Build Jade MSI installer (Windows Installer package).
# Requires: WiX Toolset (candle.exe, light.exe) and a built jade.exe.
# Usage: .\build-msi.ps1 [-Arch x86_64|i686|aarch64]
# Default arch: x86_64 (binary from target\release\ or target\<target>\release\)

param([string]$Arch = "x86_64")

$ErrorActionPreference = "Stop"
$version = "1.0.0"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$jadeLang = Split-Path -Parent (Split-Path -Parent $scriptDir)
if (-not (Test-Path (Join-Path $jadeLang "Cargo.toml"))) {
    $jadeLang = $scriptDir
    while (-not (Test-Path (Join-Path $jadeLang "Cargo.toml"))) {
        $jadeLang = Split-Path -Parent $jadeLang
        if (-not $jadeLang) { throw "Cannot find jade-lang (Cargo.toml)" }
    }
}
$repoRoot = Split-Path -Parent $jadeLang
$targetMap = @{
    "x86_64" = "x86_64-pc-windows-msvc"
    "i686"   = "i686-pc-windows-msvc"
    "aarch64" = "aarch64-pc-windows-msvc"
}
$rustTarget = $targetMap[$Arch]
if (-not $rustTarget) { $rustTarget = "x86_64-pc-windows-msvc"; $Arch = "x86_64" }
if ($Arch -eq "x86_64") {
    $binaryPath = Join-Path $jadeLang "target\release\jade.exe"
} else {
    $binaryPath = Join-Path $jadeLang "target\$rustTarget\release\jade.exe"
}
$outDir = Join-Path $repoRoot "dist\installers\windows"
$wxsPath = Join-Path $scriptDir "jade.wxs"
$msiName = "jade-$version-windows-$Arch.msi"

if (-not (Test-Path $binaryPath)) {
    Write-Host "Build the binary first: cd jade-lang; cargo build --release --target $rustTarget"
    exit 1
}

$wixBin = $null
$pathList = @("C:\Program Files (x86)\WiX Toolset v3.14\bin", "C:\Program Files\WiX Toolset v3.14\bin")
$cmd = Get-Command candle.exe -ErrorAction SilentlyContinue
if ($cmd -and $cmd.Source) { $pathList += (Split-Path $cmd.Source) }
foreach ($base in $pathList) {
    if ($base -and (Test-Path (Join-Path $base "candle.exe"))) { $wixBin = $base; break }
}
if (-not $wixBin) {
    Write-Host "WiX Toolset not found. Install from: https://wixtoolset.org/docs/wix3/"
    Write-Host "Or run as Administrator: winget install WiXToolset.WiXToolset"
    exit 1
}
$candle = Join-Path $wixBin "candle.exe"
$light = Join-Path $wixBin "light.exe"

New-Item -ItemType Directory -Force -Path $outDir | Out-Null
Push-Location $scriptDir
$wixobj = Join-Path $env:TEMP "jade-$Arch.wixobj"
$shortcutSuffix = switch ($Arch) { "x86_64" { "(64-bit)" }; "i686" { "(32-bit)" }; "aarch64" { "(ARM64)" }; default { "(64-bit)" } }
$binaryPathFull = (Resolve-Path $binaryPath).Path
& $candle -out $wixobj -dJadeExePath="$binaryPathFull" -dShortcutSuffix="$shortcutSuffix" $wxsPath
if ($LASTEXITCODE -ne 0) { Pop-Location; exit 1 }
& $light -ext WixUIExtension -out (Join-Path $outDir $msiName) $wixobj
$ok = $LASTEXITCODE -eq 0
Pop-Location
if ($ok) { Write-Host "Created: $outDir\$msiName" } else { exit 1 }
