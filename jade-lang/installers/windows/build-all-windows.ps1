# Build Jade for all Windows architectures: x86_64, i686 (32-bit), aarch64 (ARM64).
# Produces: setup.exe, .msi, and portable.zip for each arch.
# Requires: Rust (with targets: rustup target add i686-pc-windows-msvc aarch64-pc-windows-msvc), Inno Setup, WiX Toolset.

param(
    [string[]]$Archs = @("x86_64", "i686", "aarch64"),
    [switch]$SkipBuild
)

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

$targetMap = @{
    "x86_64" = "x86_64-pc-windows-msvc"
    "i686"   = "i686-pc-windows-msvc"
    "aarch64" = "aarch64-pc-windows-msvc"
}

foreach ($arch in $Archs) {
    $target = $targetMap[$arch]
    if (-not $target) { Write-Warning "Unknown arch: $arch"; continue }

    Write-Host "=== Building Jade for $arch ($target) ===" -ForegroundColor Cyan
    if (-not $SkipBuild) {
        Push-Location $jadeLang
        cargo build --release --target $target
        if ($LASTEXITCODE -ne 0) { Pop-Location; exit 1 }
        Pop-Location
    }

    Write-Host "Building installers for $arch..." -ForegroundColor Green
    & (Join-Path $scriptDir "build-exe.ps1") -Arch $arch
    if ($LASTEXITCODE -ne 0) { Write-Warning "build-exe failed for $arch" }
    & (Join-Path $scriptDir "build-msi.ps1") -Arch $arch
    if ($LASTEXITCODE -ne 0) { Write-Warning "build-msi failed for $arch" }
    & (Join-Path $scriptDir "build-portable.ps1") -Arch $arch
    if ($LASTEXITCODE -ne 0) { Write-Warning "build-portable failed for $arch" }
}

$repoRoot = Split-Path -Parent $jadeLang
$outDir = Join-Path $repoRoot "dist\installers\windows"
Write-Host "Done. Outputs in: $outDir" -ForegroundColor Cyan
Get-ChildItem $outDir -ErrorAction SilentlyContinue | ForEach-Object { Write-Host "  $($_.Name)" }
