# Build Jade for Windows ARM64 (aarch64-pc-windows-msvc).
# The 'ring' crate (used by rustls/reqwest) needs a C compiler for the target.
# This script finds Visual Studio, runs vcvarsall.bat x64_arm64 to set up the
# ARM64 cross-compiler (cl.exe), then runs cargo build.
#
# Prerequisites:
#   - Visual Studio 2022 (or 2019) with "MSVC v143 - VS 2022 C++ ARM64/ARM64EC build tools"
#     and "Windows 11 SDK" (or Windows 10 SDK). Install via Visual Studio Installer ->
#     Modify -> Individual components -> search "ARM64".
#   - rustup target add aarch64-pc-windows-msvc
#
# Usage: .\build-aarch64-with-vs.ps1

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

# Find vcvarsall.bat via vswhere (installed with VS)
$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (-not (Test-Path $vswhere)) {
    Write-Host "vswhere not found. Install Visual Studio (Build Tools or full IDE)." -ForegroundColor Red
    exit 1
}
$installPath = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath 2>$null
if (-not $installPath) {
    # Try with ARM64 component
    $installPath = & $vswhere -latest -products * -property installationPath 2>$null
}
if (-not $installPath) {
    Write-Host "Visual Studio installation not found. Install VS with C++ and ARM64 build tools." -ForegroundColor Red
    exit 1
}

$vcvarsall = Join-Path $installPath "VC\Auxiliary\Build\vcvarsall.bat"
if (-not (Test-Path $vcvarsall)) {
    Write-Host "vcvarsall.bat not found at: $vcvarsall" -ForegroundColor Red
    Write-Host "Install 'MSVC v143 - VS 2022 C++ ARM64/ARM64EC build tools' in VS Installer -> Individual components." -ForegroundColor Yellow
    exit 1
}

Write-Host "Using: $vcvarsall" -ForegroundColor Cyan
Write-Host "Setting up ARM64 cross-compiler (x64_arm64) and building..." -ForegroundColor Cyan

# Ring requires *clang* for aarch64-pc-windows-msvc (not MSVC cl). Find LLVM/Clang and set CC.
$clangBin = $null
$vsRoot = Split-Path -Parent (Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $vcvarsall)))
$vsClang = Join-Path $vsRoot "VC\Tools\Llvm\bin\clang-cl.exe"
if (Test-Path $vsClang) { $clangBin = Split-Path -Parent $vsClang }
if (-not $clangBin) {
    $pf = [Environment]::GetFolderPath("ProgramFiles")
    $llvmPath = Join-Path $pf "LLVM\bin\clang.exe"
    if (Test-Path $llvmPath) { $clangBin = Split-Path -Parent $llvmPath }
}
if (-not $clangBin) {
    $clangExe = Get-Command clang -ErrorAction SilentlyContinue
    if ($clangExe) { $clangBin = Split-Path $clangExe.Source }
}
if (-not $clangBin) {
    Write-Host "Clang is required for Windows ARM64 (ring crate). Install one of:" -ForegroundColor Red
    Write-Host "  1. Visual Studio Installer -> Modify -> Individual components -> 'C++ Clang compiler for Windows'" -ForegroundColor Yellow
    Write-Host "  2. LLVM: winget install LLVM.LLVM  (then add C:\Program Files\LLVM\bin to PATH)" -ForegroundColor Yellow
    Write-Host "  3. Or: choco install llvm" -ForegroundColor Yellow
    exit 1
}
# Ensure ARM64 linker (link.exe) and libs are first in PATH after vcvarsall, so final link uses ARM64.
$vcDir = Split-Path -Parent (Split-Path -Parent (Split-Path -Parent $vcvarsall))
$msvcDir = Join-Path $vcDir "Tools\MSVC"
$arm64Bin = $null
# We need Hostx64\arm64 and lib\arm64 for aarch64 (64-bit ARM). "arm" folder is 32-bit ARM.
if (Test-Path $msvcDir) {
    foreach ($v in (Get-ChildItem $msvcDir -Directory | Sort-Object Name -Descending)) {
        $binDir = Join-Path $v.FullName "bin"
        foreach ($hostDir in @("HostX64", "Hostx64")) {
            $linker = Join-Path $binDir "$hostDir\arm64\link.exe"
            if (Test-Path $linker) { $arm64Bin = Split-Path -Parent $linker; $arm64Lib = Join-Path $v.FullName "lib\arm64"; break }
            if ($arm64Bin) { break }
        }
        if ($arm64Bin) { break }
    }
}
if (-not $arm64Bin) {
    Write-Host "ARM64 linker (Hostx64\arm64\link.exe) not found." -ForegroundColor Red
    Write-Host "In VS Installer -> Modify -> Individual components, install 'MSVC v143 - VS 2022 C++ ARM64/ARM64EC build tools' (not 32-bit ARM)." -ForegroundColor Yellow
    exit 1
}
# So rustc uses the ARM64 linker, set it in .cargo/config.toml
$cargoDir = Join-Path $jadeLang ".cargo"
$configPath = Join-Path $cargoDir "config.toml"
$linkerPath = (Join-Path $arm64Bin "link.exe") -replace '\\', '/'
$configContent = @"
# Used by build-aarch64-with-vs.ps1 for Windows ARM64 cross-compile
[target.aarch64-pc-windows-msvc]
linker = "$linkerPath"
"@
New-Item -ItemType Directory -Force -Path $cargoDir | Out-Null
if (Test-Path $configPath) {
    $existing = Get-Content $configPath -Raw
    if ($existing -notmatch '\[target\.aarch64-pc-windows-msvc\]') {
        Add-Content -Path $configPath -Value "`n$configContent"
    }
} else {
    Set-Content -Path $configPath -Value $configContent -Encoding UTF8
}
Write-Host "Using Clang at: $clangBin" -ForegroundColor Cyan
Write-Host "Using ARM64 linker at: $arm64Bin" -ForegroundColor Cyan
$pathPrepend = "$arm64Bin;$clangBin"
# Prepend MSVC lib\arm64 so linker finds msvcrt.lib (vcvarsall may not add it)
$libPrepend = if ($arm64Lib -and (Test-Path $arm64Lib)) { "set `"LIB=$arm64Lib;%LIB%`"`r`n" } else { "" }
$batchContent = @"
@echo off
call "$vcvarsall" x64_arm64
if errorlevel 1 exit /b 1
set "PATH=$pathPrepend;%PATH%"
$libPrepend
set "CC=clang-cl"
set "CXX=clang-cl"
set "CFLAGS_aarch64_pc_windows_msvc=--target=aarch64-pc-windows-msvc"
cd /d "$jadeLang"
cargo build --release --target aarch64-pc-windows-msvc
exit /b %ERRORLEVEL%
"@
$batchPath = Join-Path $env:TEMP "jade-build-aarch64.cmd"
$batchContent | Out-File -FilePath $batchPath -Encoding ASCII
try {
    & cmd /c $batchPath
    $exitCode = $LASTEXITCODE
} finally {
    Remove-Item $batchPath -Force -ErrorAction SilentlyContinue
}
if ($exitCode -ne 0) {
    Write-Host "Build failed (exit $exitCode). Ensure 'ARM64/ARM64EC build tools' are installed in Visual Studio." -ForegroundColor Red
    exit 1
}

$binaryPath = Join-Path $jadeLang "target\aarch64-pc-windows-msvc\release\jade.exe"
if (Test-Path $binaryPath) {
    Write-Host "Built: $binaryPath" -ForegroundColor Green
    Write-Host "Run installers: .\build-exe.ps1 -Arch aarch64; .\build-msi.ps1 -Arch aarch64; .\build-portable.ps1 -Arch aarch64" -ForegroundColor Cyan
}
exit 0
