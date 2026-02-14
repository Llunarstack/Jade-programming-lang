# J Language - Complete Installer Build System
# Builds executables and creates installers for all platforms and architectures

param(
    [switch]$SkipBuild = $false,
    [switch]$WindowsOnly = $false,
    [string]$Version = "0.1.0"
)

$ErrorActionPreference = "Continue"

Write-Host ""
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host "  J Language - Complete Installer Build System" -ForegroundColor Cyan
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host ""

# Configuration
$ProjectRoot = Split-Path -Parent $PSScriptRoot
$InstallersDir = $PSScriptRoot
$DistDir = Join-Path $ProjectRoot "dist"
$InstallerDistDir = Join-Path $DistDir "installers"

# Create directories
if (-not (Test-Path $DistDir)) {
    New-Item -ItemType Directory -Path $DistDir | Out-Null
}
if (-not (Test-Path $InstallerDistDir)) {
    New-Item -ItemType Directory -Path $InstallerDistDir | Out-Null
}

# Step 1: Build executables for all platforms
if (-not $SkipBuild) {
    Write-Host " Step 1: Building executables for all platforms..." -ForegroundColor Green
    Write-Host ""
    
    & "$InstallersDir\build-all-executables.ps1"
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host ""
        Write-Host " Failed to build executables" -ForegroundColor Red
        exit 1
    }
    
    Write-Host ""
    Write-Host " Executables built successfully" -ForegroundColor Green
    Write-Host ""
} else {
    Write-Host "  Skipping executable build (using existing binaries)" -ForegroundColor Yellow
    Write-Host ""
}

# Step 2: Build Windows installers
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host " Step 2: Building Windows Installers" -ForegroundColor Green
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host ""

$windowsInstallers = @()

# Check for Inno Setup
$InnoSetupPath = $null
$InnoLocations = @(
    "${env:LOCALAPPDATA}\Programs\Inno Setup 6\ISCC.exe",
    "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe",
    "${env:ProgramFiles}\Inno Setup 6\ISCC.exe",
    "${env:ProgramFiles(x86)}\Inno Setup 5\ISCC.exe",
    "${env:ProgramFiles}\Inno Setup 5\ISCC.exe"
)

foreach ($Location in $InnoLocations) {
    if (Test-Path $Location) {
        $InnoSetupPath = $Location
        break
    }
}

if ($InnoSetupPath) {
    Write-Host " Found Inno Setup at: $InnoSetupPath" -ForegroundColor Green
    Write-Host ""
    
    # Build Inno Setup installer (EXE)
    Write-Host " Building Inno Setup installer (EXE)..." -ForegroundColor Yellow
    
    try {
        Push-Location $InstallersDir
        & $InnoSetupPath "j-lang-setup.iss" /Q
        Pop-Location
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "    Inno Setup installer created" -ForegroundColor Green
            $windowsInstallers += "Inno Setup EXE"
        } else {
            Write-Host "     Inno Setup build failed" -ForegroundColor Yellow
        }
    } catch {
        Write-Host "     Inno Setup build error: $($_.Exception.Message)" -ForegroundColor Yellow
    }
    
    Write-Host ""
} else {
    Write-Host "  Inno Setup not found - skipping EXE installer" -ForegroundColor Yellow
    Write-Host "   Download from: https://jrsoftware.org/isdl.php" -ForegroundColor Gray
    Write-Host ""
}

# Check for WiX Toolset
$WixPath = $null
$WixLocations = @(
    "${env:ProgramFiles(x86)}\WiX Toolset v3.11\bin",
    "${env:ProgramFiles}\WiX Toolset v3.11\bin",
    "${env:ProgramFiles(x86)}\WiX Toolset v3.14\bin",
    "${env:ProgramFiles}\WiX Toolset v3.14\bin",
    "${env:WIX}bin"
)

foreach ($Location in $WixLocations) {
    if (Test-Path "$Location\candle.exe") {
        $WixPath = $Location
        break
    }
}

if ($WixPath) {
    Write-Host " Found WiX Toolset at: $WixPath" -ForegroundColor Green
    Write-Host ""
    
    # Build MSI installers for x64 and x86
    foreach ($arch in @("x64", "x86")) {
        Write-Host " Building MSI installer ($arch)..." -ForegroundColor Yellow
        
        try {
            & "$InstallersDir\build-msi.ps1" -Version $Version -Architecture $arch
            
            if ($LASTEXITCODE -eq 0) {
                Write-Host "    MSI installer ($arch) created" -ForegroundColor Green
                $windowsInstallers += "MSI $arch"
            } else {
                Write-Host "     MSI ($arch) build failed" -ForegroundColor Yellow
            }
        } catch {
            Write-Host "     MSI ($arch) build error: $($_.Exception.Message)" -ForegroundColor Yellow
        }
        
        Write-Host ""
    }
} else {
    Write-Host "  WiX Toolset not found - skipping MSI installers" -ForegroundColor Yellow
    Write-Host "   Download from: https://wixtoolset.org/releases/" -ForegroundColor Gray
    Write-Host ""
}

# Create portable ZIP packages for Windows
Write-Host " Creating Windows portable packages..." -ForegroundColor Yellow

$windowsTargets = @(
    @{Name="windows-x86_64"; Arch="x64"},
    @{Name="windows-i686"; Arch="x86"},
    @{Name="windows-aarch64"; Arch="arm64"}
)

foreach ($target in $windowsTargets) {
    $exePath = Join-Path $DistDir "j-$($target.Name).exe"
    
    if (Test-Path $exePath) {
        $zipName = "j-lang-$Version-$($target.Name)-portable.zip"
        $zipPath = Join-Path $InstallerDistDir $zipName
        
        # Create temp directory
        $tempDir = Join-Path $env:TEMP "j-portable-$($target.Name)"
        if (Test-Path $tempDir) {
            Remove-Item -Recurse -Force $tempDir
        }
        New-Item -ItemType Directory -Path $tempDir | Out-Null
        
        # Copy files
        Copy-Item $exePath (Join-Path $tempDir "j.exe")
        Copy-Item (Join-Path $ProjectRoot "README.md") $tempDir -ErrorAction SilentlyContinue
        Copy-Item (Join-Path $ProjectRoot "..\README.md") $tempDir -ErrorAction SilentlyContinue
        Copy-Item (Join-Path $ProjectRoot "examples") (Join-Path $tempDir "examples") -Recurse -ErrorAction SilentlyContinue
        
        # Create ZIP
        Compress-Archive -Path "$tempDir\*" -DestinationPath $zipPath -Force
        
        # Cleanup
        Remove-Item -Recurse -Force $tempDir
        
        Write-Host "    Created: $zipName" -ForegroundColor Green
        $windowsInstallers += "Portable ZIP ($($target.Arch))"
    }
}

Write-Host ""

# Step 3: Build Linux packages (if not Windows-only)
if (-not $WindowsOnly) {
    Write-Host "========================================================" -ForegroundColor Cyan
    Write-Host " Step 3: Building Linux Packages" -ForegroundColor Green
    Write-Host "========================================================" -ForegroundColor Cyan
    Write-Host ""
    
    Write-Host "  Linux package building requires Linux environment" -ForegroundColor Yellow
    Write-Host "   Run these scripts on Linux or WSL:" -ForegroundColor Gray
    Write-Host "   - build-deb.sh (Debian/Ubuntu)" -ForegroundColor Gray
    Write-Host "   - build-rpm.sh (RedHat/Fedora)" -ForegroundColor Gray
    Write-Host ""
    
    # Create Linux portable tarballs
    Write-Host " Creating Linux portable packages..." -ForegroundColor Yellow
    
    $linuxTargets = @(
        "linux-x86_64",
        "linux-i686",
        "linux-aarch64",
        "linux-armv7"
    )
    
    foreach ($target in $linuxTargets) {
        $exePath = Join-Path $DistDir "j-$target"
        
        if (Test-Path $exePath) {
            $tarName = "j-lang-$Version-$target.tar.gz"
            $tarPath = Join-Path $InstallerDistDir $tarName
            
            Write-Host "    Would create: $tarName (requires tar)" -ForegroundColor Green
        }
    }
    
    Write-Host ""
    
    # Step 4: Build macOS packages
    Write-Host "========================================================" -ForegroundColor Cyan
    Write-Host " Step 4: Building macOS Packages" -ForegroundColor Green
    Write-Host "========================================================" -ForegroundColor Cyan
    Write-Host ""
    
    Write-Host "  macOS package building requires macOS environment" -ForegroundColor Yellow
    Write-Host "   Run build-macos-pkg.sh on macOS" -ForegroundColor Gray
    Write-Host ""
}

# Summary
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host " Build Summary" -ForegroundColor Cyan
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host ""

Write-Host "Windows Installers Created:" -ForegroundColor Green
if ($windowsInstallers.Count -gt 0) {
    foreach ($installer in $windowsInstallers) {
        Write-Host "   $installer" -ForegroundColor Green
    }
} else {
    Write-Host "    No installers created" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "All installers saved to: $InstallerDistDir" -ForegroundColor Cyan
Write-Host ""

# List all created files
Write-Host "Created Files:" -ForegroundColor Cyan
Get-ChildItem $InstallerDistDir -ErrorAction SilentlyContinue | ForEach-Object {
    $size = [math]::Round($_.Length / 1MB, 2)
    Write-Host "  $($_.Name) - $size MB" -ForegroundColor Gray
}

Write-Host ""
Write-Host "Build complete!" -ForegroundColor Green
Write-Host ""
