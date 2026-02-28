# Build Jade .exe installer (Inno Setup).
# Usage: .\build-exe.ps1 [-Arch x86_64|i686|aarch64]
param([string]$Arch = "x86_64")

$ErrorActionPreference = "Stop"
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$jadeLang = Split-Path -Parent (Split-Path -Parent $scriptDir)
if (-not (Test-Path (Join-Path $jadeLang "Cargo.toml"))) {
    $jadeLang = $scriptDir
    while (-not (Test-Path (Join-Path $jadeLang "Cargo.toml"))) {
        $jadeLang = Split-Path -Parent $jadeLang
        if (-not $jadeLang) { throw "Cannot find jade-lang (Cargo.toml)" }
    }
}
$targetMap = @{ "x86_64" = "x86_64-pc-windows-msvc"; "i686" = "i686-pc-windows-msvc"; "aarch64" = "aarch64-pc-windows-msvc" }
$rustTarget = $targetMap[$Arch]; if (-not $rustTarget) { $rustTarget = "x86_64-pc-windows-msvc"; $Arch = "x86_64" }
$binaryPath = if ($Arch -eq "x86_64") { Join-Path $jadeLang "target\release\jade.exe" } else { Join-Path $jadeLang "target\$rustTarget\release\jade.exe" }
if (-not (Test-Path $binaryPath)) {
    Write-Host "Build the binary first: cd jade-lang; cargo build --release --target $rustTarget"
    exit 1
}

$iscc = $null
$paths = @(
    "$env:LOCALAPPDATA\Programs\Inno Setup 6\ISCC.exe",
    "C:\Program Files (x86)\Inno Setup 6\ISCC.exe",
    "C:\Program Files\Inno Setup 6\ISCC.exe",
    (Get-Command iscc.exe -ErrorAction SilentlyContinue).Source
)
foreach ($p in $paths) {
    if ($p -and (Test-Path $p)) { $iscc = $p; break }
}
if (-not $iscc) {
    foreach ($base in @([Environment]::GetFolderPath('ProgramFilesX86'), [Environment]::GetFolderPath('ProgramFiles'))) {
        Get-ChildItem $base -Directory -Filter "*Inno*" -ErrorAction SilentlyContinue | ForEach-Object {
            $c = Join-Path $_.FullName "ISCC.exe"
            if (Test-Path $c) { $iscc = $c; break }
        }
        if ($iscc) { break }
    }
}
if (-not $iscc) {
    Write-Host "Inno Setup not found. Install from: https://jrsoftware.org/isinfo.php"
    Write-Host "Or: winget install JRSoftware.InnoSetup"
    exit 1
}

$repoRoot = Split-Path -Parent $jadeLang
$outDir = Join-Path $repoRoot "dist\installers\windows"
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

$version = "1.0.0"
$shortcutSuffix = switch ($Arch) { "x86_64" { "(64-bit)" }; "i686" { "(32-bit)" }; "aarch64" { "(ARM64)" }; default { "(64-bit)" } }
# Use absolute paths so the installer always finds the exe and IDE extension.
$binaryPathFull = (Resolve-Path $binaryPath).Path
$ideExtPath = Join-Path $jadeLang "installers\ide\vscode-snippet"
if (-not (Test-Path $ideExtPath)) {
    Write-Host "WARNING: IDE extension folder not found: $ideExtPath"
} else {
    Write-Host "IDE extension: $ideExtPath"
}
$binaryDir = Split-Path -Parent $binaryPath
$buildStamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
Write-Host "Packaging: $binaryPathFull"
Write-Host "Build stamp: $buildStamp"
Push-Location $scriptDir
# Write build stamp for installer to include (verifies install).
Set-Content -Path ".\install-extras\BUILD.txt" -Value "Jade $version - built $buildStamp - $Arch"
& $iscc "jade-setup.iss" "/DJadeExePath=`"$binaryPathFull`"" "/DBinaryRelPath=$binaryDir" "/DIdeExtPath=`"$ideExtPath`"" "/DOutputArch=$Arch" "/DShortcutSuffix=$shortcutSuffix" "/DBuildStamp=$buildStamp"
$ok = $LASTEXITCODE -eq 0
Pop-Location
if ($ok) { Write-Host "Created: $outDir\jade-$version-windows-$Arch-setup.exe" } else { exit 1 }
