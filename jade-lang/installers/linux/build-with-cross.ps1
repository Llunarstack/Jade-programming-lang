# Build Jade for Linux (x86_64) from Windows using cross (Docker).
# The 'ring' crate needs a C compiler for the target; cross runs the build
# inside a Linux container that has gcc, so no local Linux toolchain is needed.
#
# Prerequisites:
#   - Docker Desktop installed and running (https://www.docker.com/products/docker-desktop/)
#   - cargo install cross
#   - rustup target add x86_64-unknown-linux-gnu
#
# Usage: .\build-with-cross.ps1   (run from this directory or repo root)
# Output: jade-lang/target/x86_64-unknown-linux-gnu/release/jade (Linux binary)

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

# Check for cross
$cross = Get-Command cross -ErrorAction SilentlyContinue
if (-not $cross) {
    Write-Host "cross not found. Install with: cargo install cross" -ForegroundColor Red
    Write-Host "Then ensure Docker Desktop is installed and running." -ForegroundColor Yellow
    exit 1
}

# Optional: check Docker
$docker = Get-Command docker -ErrorAction SilentlyContinue
if (-not $docker) {
    Write-Host "Docker not found. Install Docker Desktop and ensure it is running." -ForegroundColor Yellow
}

Write-Host "Building for x86_64-unknown-linux-gnu with cross (Docker)..." -ForegroundColor Cyan
Push-Location $jadeLang
try {
    & cross build --release --target x86_64-unknown-linux-gnu
    if ($LASTEXITCODE -ne 0) { exit 1 }
    $out = Join-Path $jadeLang "target\x86_64-unknown-linux-gnu\release\jade"
    if (Test-Path $out) {
        Write-Host "Built Linux binary: $out" -ForegroundColor Green
        Write-Host "To create .deb/.rpm, copy this binary to a Linux machine and run build-deb.sh / build-rpm.sh there." -ForegroundColor Cyan
    }
} finally {
    Pop-Location
}
