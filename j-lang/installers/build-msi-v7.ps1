# J Language - Build MSI with WiX v7.0
# Uses the new WiX v7.0 command-line interface

param(
    [string]$Version = "0.1.0",
    [string]$Architecture = "x64"
)

$ErrorActionPreference = "Continue"

Write-Host ""
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host "  J Language - MSI Builder (WiX v7.0)" -ForegroundColor Cyan
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host ""

# Check for WiX v7.0
$WixPath = "C:\Program Files\WiX Toolset v7.0\bin\wix.exe"

if (-not (Test-Path $WixPath)) {
    Write-Host "ERROR: WiX Toolset v7.0 not found at: $WixPath" -ForegroundColor Red
    Write-Host ""
    Write-Host "Please install WiX Toolset v7.0 from:" -ForegroundColor Yellow
    Write-Host "  https://wixtoolset.org/releases/" -ForegroundColor Blue
    Write-Host ""
    exit 1
}

Write-Host "Found WiX at: $WixPath" -ForegroundColor Green
Write-Host ""

# Configuration
$ProjectRoot = Split-Path -Parent $PSScriptRoot
$InstallersDir = $PSScriptRoot
$DistDir = Join-Path $ProjectRoot "dist"
$InstallerDistDir = Join-Path $DistDir "installers"

# Create directories
if (-not (Test-Path $InstallerDistDir)) {
    New-Item -ItemType Directory -Path $InstallerDistDir | Out-Null
}

# Check for executable
$ExePath = Join-Path $DistDir "j-windows-$Architecture.exe"
if (-not (Test-Path $ExePath)) {
    $ExePath = Join-Path $DistDir "j-windows-x86_64.exe"
    if (-not (Test-Path $ExePath)) {
        Write-Host ""
        Write-Host "ERROR: Executable not found" -ForegroundColor Red
        Write-Host "Expected: $ExePath" -ForegroundColor Yellow
        Write-Host ""
        Write-Host "Please build first:" -ForegroundColor Yellow
        Write-Host "  cd .." -ForegroundColor Blue
        Write-Host "  cargo build --release" -ForegroundColor Blue
        Write-Host "  Copy-Item target\release\j.exe dist\j-windows-x86_64.exe" -ForegroundColor Blue
        Write-Host ""
        exit 1
    }
}

Write-Host "Using executable: $ExePath" -ForegroundColor Green
Write-Host ""

# Build MSI using WiX v7.0
Write-Host "Building MSI installer..." -ForegroundColor Blue
Write-Host ""

$WxsFile = Join-Path $InstallersDir "j-lang.wxs"
$MsiFile = Join-Path $InstallerDistDir "j-lang-$Version-windows-$Architecture.msi"

try {
    # WiX v7.0 uses a single command
    Write-Host "Compiling with WiX v7.0..." -ForegroundColor Blue
    
    & $WixPath build -arch $Architecture -out $MsiFile $WxsFile
    
    if ($LASTEXITCODE -ne 0) {
        throw "WiX build failed with exit code $LASTEXITCODE"
    }
    
    # Success
    Write-Host ""
    Write-Host "========================================================" -ForegroundColor Cyan
    Write-Host "  MSI Installer Created Successfully!" -ForegroundColor Green
    Write-Host "========================================================" -ForegroundColor Cyan
    Write-Host ""
    
    if (Test-Path $MsiFile) {
        $MsiSize = [math]::Round((Get-Item $MsiFile).Length / 1MB, 2)
        Write-Host "File: $MsiFile" -ForegroundColor Cyan
        Write-Host "Size: $MsiSize MB" -ForegroundColor Cyan
        Write-Host ""
        
        Write-Host "To install:" -ForegroundColor Yellow
        Write-Host "  msiexec /i `"$MsiFile`"" -ForegroundColor Blue
        Write-Host ""
        Write-Host "Or double-click the MSI file" -ForegroundColor Gray
        Write-Host ""
    } else {
        Write-Host "WARNING: MSI file not found after build" -ForegroundColor Yellow
    }
    
} catch {
    Write-Host ""
    Write-Host "ERROR: Failed to build MSI" -ForegroundColor Red
    Write-Host $_.Exception.Message -ForegroundColor Red
    Write-Host ""
    Write-Host "Note: WiX v7.0 has a different syntax than v3.x" -ForegroundColor Yellow
    Write-Host "You may need to update the .wxs file for v7.0 compatibility" -ForegroundColor Yellow
    Write-Host ""
    exit 1
}
