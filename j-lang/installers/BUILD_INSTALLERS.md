# J Language - Building Installers

Professional installer build system for all platforms and architectures.

## Quick Start

### Build Everything (Windows)

```powershell
.\build-all-installers.ps1
```

This will:
1. Build executables for all platforms (Windows, Linux, macOS)
2. Create Windows installers (MSI, EXE, ZIP)
3. Generate checksums

### Build Only Executables

```powershell
.\build-all-executables.ps1
```

### Build Specific Installer

```powershell
# Inno Setup (EXE)
iscc j-lang-setup.iss

# WiX (MSI) - 64-bit
.\build-msi.ps1 -Architecture x64

# WiX (MSI) - 32-bit
.\build-msi.ps1 -Architecture x86
```

## Prerequisites

### Windows

**Required:**
- Visual Studio Build Tools with C++ support
- Rust toolchain (rustup) - https://rustup.rs/

**For MSI installers:**
- WiX Toolset v3.11+ - https://wixtoolset.org/releases/

**For EXE installers:**
- Inno Setup 6+ - https://jrsoftware.org/isdl.php

### Linux

```bash
# Debian/Ubuntu
sudo apt install build-essential dpkg-dev

# RedHat/Fedora
sudo dnf install rpm-build rpmdevtools
```

### macOS

```bash
xcode-select --install
```

## Supported Platforms

### Windows
- **x86_64** (64-bit) - MSI, EXE, ZIP
- **i686** (32-bit) - MSI, EXE, ZIP
- **aarch64** (ARM64) - ZIP only

### Linux
- **x86_64** (64-bit) - DEB, RPM, tar.gz
- **i686** (32-bit) - DEB, RPM, tar.gz
- **aarch64** (ARM64) - tar.gz
- **armv7** (ARM) - tar.gz

### macOS
- **x86_64** (Intel) - PKG, tar.gz
- **aarch64** (Apple Silicon) - PKG, tar.gz

## Output Structure

```
dist/
├── installers/
│   ├── j-lang-0.1.0-windows-setup.exe          # Inno Setup installer
│   ├── j-lang-0.1.0-windows-x64.msi            # WiX 64-bit MSI
│   ├── j-lang-0.1.0-windows-x86.msi            # WiX 32-bit MSI
│   ├── j-lang-0.1.0-windows-x86_64-portable.zip
│   ├── j-lang-0.1.0-windows-i686-portable.zip
│   ├── j-lang-0.1.0-windows-aarch64-portable.zip
│   ├── j-lang-0.1.0-linux-x86_64.deb
│   ├── j-lang-0.1.0-linux-x86_64.rpm
│   ├── j-lang-0.1.0-macos-x86_64.pkg
│   └── checksums.txt
├── j-windows-x86_64.exe                        # Raw executables
├── j-windows-i686.exe
├── j-linux-x86_64
├── j-macos-x86_64
└── checksums.txt
```

## Installer Features

### Windows Inno Setup (EXE)

- Modern GUI wizard
- Custom install location
- Add to PATH option
- .j file association
- Desktop shortcuts
- Start menu entries
- Uninstaller in Add/Remove Programs

### Windows WiX (MSI)

- Enterprise-grade MSI package
- Group Policy deployment support
- Silent installation: `msiexec /i installer.msi /quiet`
- Add to PATH
- .j file association
- Start menu entries
- Proper uninstall support

### Windows Portable (ZIP)

- No installation required
- Extract and run
- Includes examples and documentation
- Perfect for USB drives

### Linux DEB

- Standard Debian package
- Automatic dependency resolution
- System-wide installation to /usr/local/bin
- Man pages included
- Install: `sudo dpkg -i j-lang.deb`

### Linux RPM

- Standard RPM package
- Automatic dependency resolution
- System-wide installation
- Man pages included
- Install: `sudo rpm -i j-lang.rpm`

### macOS PKG

- Native macOS installer
- Installs to /usr/local/bin
- Includes uninstaller script
- Install: `sudo installer -pkg j-lang.pkg -target /`

## Build Scripts

### build-all-installers.ps1

Master script that builds everything.

**Options:**
```powershell
# Skip executable build (use existing)
.\build-all-installers.ps1 -SkipBuild

# Build Windows installers only
.\build-all-installers.ps1 -WindowsOnly

# Specify version
.\build-all-installers.ps1 -Version "0.2.0"
```

### build-all-executables.ps1

Builds release binaries for all platforms.

**Options:**
```powershell
# Skip tests
.\build-all-executables.ps1 -SkipTests
```

### build-msi.ps1

Builds Windows MSI installer using WiX.

**Options:**
```powershell
# 64-bit
.\build-msi.ps1 -Architecture x64

# 32-bit
.\build-msi.ps1 -Architecture x86

# Custom version
.\build-msi.ps1 -Version "0.2.0" -Architecture x64
```

## Troubleshooting

### "WiX Toolset not found"

**Solution:**
1. Download from https://wixtoolset.org/releases/
2. Install WiX Toolset v3.11 or later
3. Ensure `candle.exe` and `light.exe` are in PATH
4. Or set `WIX` environment variable to WiX installation directory

### "Inno Setup not found"

**Solution:**
1. Download from https://jrsoftware.org/isdl.php
2. Install Inno Setup 6 or later
3. Ensure `iscc.exe` is in PATH
4. Default location: `C:\Program Files (x86)\Inno Setup 6\ISCC.exe`

### "cross not found"

**Solution:**
The script will auto-install cross for cross-compilation.

Or manually:
```powershell
cargo install cross --git https://github.com/cross-rs/cross
```

### "Binary not found"

**Solution:**
Build the project first:
```powershell
cd ..
cargo build --release
cd installers
```

### Linux: "dpkg-deb: command not found"

**Solution:**
```bash
sudo apt install dpkg-dev
```

### Linux: "rpmbuild: command not found"

**Solution:**
```bash
sudo dnf install rpm-build rpmdevtools
```

### macOS: "pkgbuild: command not found"

**Solution:**
```bash
xcode-select --install
```

## CI/CD Integration

### GitHub Actions

```yaml
name: Build Installers

on:
  push:
    tags:
      - 'v*'

jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build installers
        run: |
          cd j-lang/installers
          .\build-all-installers.ps1
      - uses: actions/upload-artifact@v3
        with:
          name: windows-installers
          path: j-lang/dist/installers/*

  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build packages
        run: |
          cd j-lang/installers
          ./build-deb.sh
          ./build-rpm.sh
      - uses: actions/upload-artifact@v3
        with:
          name: linux-packages
          path: j-lang/dist/installers/*

  build-macos:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build package
        run: |
          cd j-lang/installers
          ./build-macos-pkg.sh
      - uses: actions/upload-artifact@v3
        with:
          name: macos-package
          path: j-lang/dist/installers/*
```

## Manual Installation

If automated installers don't work:

### Windows
1. Build: `cargo build --release`
2. Copy `target\release\j.exe` to `C:\Program Files\J\`
3. Add to PATH:
   - System Properties → Environment Variables
   - Edit user PATH
   - Add `C:\Program Files\J\`

### Linux/macOS
1. Build: `cargo build --release`
2. Copy: `sudo cp target/release/j /usr/local/bin/`
3. Make executable: `sudo chmod +x /usr/local/bin/j`

## Testing Installers

### Windows MSI
```powershell
# Install
msiexec /i dist\installers\j-lang-0.1.0-windows-x64.msi

# Silent install
msiexec /i dist\installers\j-lang-0.1.0-windows-x64.msi /quiet

# Uninstall
msiexec /x dist\installers\j-lang-0.1.0-windows-x64.msi
```

### Windows EXE
```powershell
# Install (GUI)
.\dist\installers\j-lang-0.1.0-windows-setup.exe

# Silent install
.\dist\installers\j-lang-0.1.0-windows-setup.exe /VERYSILENT /NORESTART

# Uninstall
# Use Add/Remove Programs or uninstaller in Start Menu
```

### Linux DEB
```bash
# Install
sudo dpkg -i dist/installers/j-lang-0.1.0-linux-x86_64.deb

# Uninstall
sudo dpkg -r j-lang
```

### Linux RPM
```bash
# Install
sudo rpm -i dist/installers/j-lang-0.1.0-linux-x86_64.rpm

# Uninstall
sudo rpm -e j-lang
```

### macOS PKG
```bash
# Install
sudo installer -pkg dist/installers/j-lang-0.1.0-macos-x86_64.pkg -target /

# Uninstall
# Run uninstall script included in package
```

## License

MIT License - See LICENSE file for details.
