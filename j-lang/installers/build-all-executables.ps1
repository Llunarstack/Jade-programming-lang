# J Language - Build All Executables
# Builds release binaries for all supported platforms

param(
    [switch]$SkipTests = $false
)

$ErrorActionPreference = "Stop"

Write-Host ""
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host "  J Language - Multi-Platform Executable Builder" -ForegroundColor Cyan
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host ""

# Configuration
$ProjectRoot = Split-Path -Parent $PSScriptRoot
$DistDir = Join-Path $ProjectRoot "dist"

# Create dist directory
if (-not (Test-Path $DistDir)) {
    New-Item -ItemType Directory -Path $DistDir | Out-Null
}

# Check for Rust/Cargo
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Cargo not found. Please install Rust from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

Write-Host "✅ Cargo found: $(cargo --version)" -ForegroundColor Green
Write-Host ""

# Run tests first (unless skipped)
if (-not $SkipTests) {
    Write-Host "🧪 Running tests..." -ForegroundColor Yellow
    Push-Location $ProjectRoot
    cargo test --release
    Pop-Location
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host ""
        Write-Host "❌ Tests failed!" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "✅ All tests passed" -ForegroundColor Green
    Write-Host ""
}

# Define build targets
$targets = @(
    # Windows - Native builds
    @{Name="windows-x86_64"; Target="x86_64-pc-windows-msvc"; Ext=".exe"; Native=$true},
    @{Name="windows-i686"; Target="i686-pc-windows-msvc"; Ext=".exe"; Native=$true},
    @{Name="windows-aarch64"; Target="aarch64-pc-windows-msvc"; Ext=".exe"; Native=$false},
    
    # Linux - Cross-compile
    @{Name="linux-x86_64"; Target="x86_64-unknown-linux-gnu"; Ext=""; Native=$false},
    @{Name="linux-i686"; Target="i686-unknown-linux-gnu"; Ext=""; Native=$false},
    @{Name="linux-aarch64"; Target="aarch64-unknown-linux-gnu"; Ext=""; Native=$false},
    @{Name="linux-armv7"; Target="armv7-unknown-linux-gnueabihf"; Ext=""; Native=$false},
    
    # macOS - Cross-compile
    @{Name="macos-x86_64"; Target="x86_64-apple-darwin"; Ext=""; Native=$false},
    @{Name="macos-aarch64"; Target="aarch64-apple-darwin"; Ext=""; Native=$false}
)

Write-Host "📦 Building for $($targets.Count) platforms..." -ForegroundColor Green
Write-Host ""

$successCount = 0
$failCount = 0
$results = @()

foreach ($target in $targets) {
    Write-Host "🔨 Building: $($target.Name) ($($target.Target))..." -ForegroundColor Yellow
    
    # Add target
    Write-Host "   Adding target..." -ForegroundColor Gray
    rustup target add $target.Target 2>&1 | Out-Null
    
    # Build
    Push-Location $ProjectRoot
    
    try {
        if ($target.Native) {
            # Native build with cargo
            Write-Host "   Compiling (native)..." -ForegroundColor Gray
            cargo build --release --target $target.Target 2>&1 | Out-Null
        } else {
            # Check for cross
            if (-not (Get-Command cross -ErrorAction SilentlyContinue)) {
                Write-Host "   ⚠️  Installing cross for cross-compilation..." -ForegroundColor Yellow
                cargo install cross --git https://github.com/cross-rs/cross 2>&1 | Out-Null
            }
            
            # Cross-compile
            Write-Host "   Compiling (cross)..." -ForegroundColor Gray
            cross build --release --target $target.Target 2>&1 | Out-Null
        }
        
        # Check if build succeeded
        $binaryPath = "target\$($target.Target)\release\j$($target.Ext)"
        
        if (Test-Path $binaryPath) {
            # Copy to dist
            $distPath = "$DistDir\j-$($target.Name)$($target.Ext)"
            Copy-Item $binaryPath $distPath -Force
            
            # Get file size
            $size = (Get-Item $distPath).Length / 1MB
            
            Write-Host "   ✅ Success! Size: $([math]::Round($size, 2)) MB" -ForegroundColor Green
            $successCount++
            $results += @{
                Platform=$target.Name
                Status="✅ Success"
                Size="$([math]::Round($size, 2)) MB"
                Path=$distPath
            }
        } else {
            Write-Host "   ❌ Failed: Binary not found at $binaryPath" -ForegroundColor Red
            $failCount++
            $results += @{
                Platform=$target.Name
                Status="❌ Failed"
                Size="N/A"
                Path=""
            }
        }
    } catch {
        Write-Host "   ❌ Failed: $($_.Exception.Message)" -ForegroundColor Red
        $failCount++
        $results += @{
            Platform=$target.Name
            Status="❌ Failed"
            Size="N/A"
            Path=""
        }
    }
    
    Pop-Location
    Write-Host ""
}

# Generate checksums
if ($successCount -gt 0) {
    Write-Host "🔐 Generating SHA256 checksums..." -ForegroundColor Yellow
    $checksumFile = Join-Path $DistDir "checksums.txt"
    
    "" | Out-File -FilePath $checksumFile -Encoding UTF8
    "J Language v0.1.0 - SHA256 Checksums" | Out-File -Append -FilePath $checksumFile -Encoding UTF8
    "Generated: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')" | Out-File -Append -FilePath $checksumFile -Encoding UTF8
    "=" * 80 | Out-File -Append -FilePath $checksumFile -Encoding UTF8
    "" | Out-File -Append -FilePath $checksumFile -Encoding UTF8
    
    Get-ChildItem $DistDir -Filter "j-*" | ForEach-Object {
        $hash = (Get-FileHash $_.FullName -Algorithm SHA256).Hash.ToLower()
        "$hash  $($_.Name)" | Out-File -Append -FilePath $checksumFile -Encoding UTF8
    }
    
    Write-Host "   ✅ Checksums saved to: checksums.txt" -ForegroundColor Green
    Write-Host ""
}

# Summary
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host "📊 Build Summary" -ForegroundColor Cyan
Write-Host "========================================================" -ForegroundColor Cyan
Write-Host ""

Write-Host "Platform                Status          Size" -ForegroundColor Cyan
Write-Host "--------------------    -------------   --------" -ForegroundColor Gray

foreach ($result in $results) {
    $color = if ($result.Status -like "*Success*") { "Green" } else { "Red" }
    $platform = $result.Platform.PadRight(20)
    $status = $result.Status.PadRight(15)
    $size = $result.Size
    
    Write-Host "$platform $status $size" -ForegroundColor $color
}

Write-Host ""
Write-Host "Total: $successCount succeeded, $failCount failed" -ForegroundColor $(if ($failCount -eq 0) { "Green" } else { "Yellow" })
Write-Host ""
Write-Host "📁 Binaries saved to: $DistDir\" -ForegroundColor Cyan
Write-Host ""

if ($successCount -gt 0) {
    Write-Host "✨ Build complete! Ready to create installers." -ForegroundColor Green
} else {
    Write-Host "⚠️  No binaries were built successfully." -ForegroundColor Yellow
}

Write-Host ""

# Return success if at least one build succeeded
exit $(if ($successCount -gt 0) { 0 } else { 1 })
