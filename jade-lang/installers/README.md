# Jade installers and language recognition

This directory contains installers and OS/IDE integration so that:

- **Windows, macOS, and Linux** get native installers (and/or install scripts) for x86_64 and ARM64 where supported.
- **Your PC and other apps** recognize `.jdl` as a programming language: file associations, MIME types, “Open with Jade”, and optional IDE support.
- **One binary** supports **interpreted** (`jade file.jdl`), **AOT compiled** (`jade build file.jdl`), and optional **JIT** (when built with `--features jit`).

## Quick install (no installer)

- **Windows:** Build with `cargo build --release` in `jade-lang`, then run `installers\windows\install.ps1` (or add `target\release\jade.exe` to PATH and run the association script).
- **macOS / Linux:** From repo root, `cd jade-lang && cargo build --release`, then run `./installers/macos/install.sh` or `./installers/linux/install.sh`.

## Build variants

| Variant        | How to build                    | Usage                    |
|----------------|----------------------------------|--------------------------|
| Interpreter    | Default (no extra features)     | `jade file.jdl`, `jade run file.jdl` |
| AOT (compiled) | Same binary; needs LLVM for full support | `jade build file.jdl -o app` |
| JIT            | `cargo build --release --features jit`  | Use if your build exposes JIT mode |

The same installer can ship the default binary; AOT and JIT are modes/features of that binary.

## Installer formats (all platforms)

| OS       | Format   | File(s) | How to build |
|----------|----------|---------|--------------|
| **Windows** | Setup (GUI) | `.exe` | `cd jade-lang/installers/windows && iscc jade-setup.iss` |
| **Windows** | MSI       | `.msi` | `./jade-lang/installers/windows/build-msi.ps1` (adds install dir to PATH via WixUtilExtension) |
| **Windows** | Portable  | `.zip` | `./jade-lang/installers/windows/build-portable.ps1` |
| **Windows** | All archs | exe+msi+zip per arch | `./jade-lang/installers/windows/build-all-windows.ps1` |
| **Linux**   | Debian/Ubuntu | `.deb` | `./jade-lang/installers/linux/build-deb.sh` |
| **Linux**   | Fedora/RHEL | `.rpm` | `./jade-lang/installers/linux/build-rpm.sh` (needs `fpm`) |
| **Linux**   | Portable  | `.AppImage` | `./jade-lang/installers/linux/build-appimage.sh` |
| **macOS**   | Package   | `.pkg` | `./jade-lang/installers/macos/build-pkg.sh` |
| **macOS**   | Disk image | `.dmg` | `./jade-lang/installers/macos/build-dmg.sh` |
| **Android** | Termux    | `.tar.gz` | `./jade-lang/installers/android/build-termux.sh` |

Build from repo root after `cd jade-lang && cargo build --release`. Outputs: `dist/installers/<platform>/`.

## Cross-compilation and C toolchain

The crate uses **ring** (via rustls/reqwest), which needs a **C compiler for the target**. So:

- **Windows ARM64** from x64 Windows: need (1) **Clang** for the `ring` crate (e.g. `winget install LLVM.LLVM`) and (2) **Visual Studio ARM64 build tools** for the linker. Run:
  - `rustup target add aarch64-pc-windows-msvc`
  - `.\jade-lang\installers\windows\build-aarch64-with-vs.ps1`  
  The script uses `vcvarsall x64_arm64`, sets `CC=clang-cl`, and points the linker to `Hostx64\arm64\link.exe`. You must install **"MSVC v143 - VS 2022 C++ ARM64/ARM64EC build tools"** (search **ARM64** in Individual components). Do not use the 32-bit **ARM** component only—that creates `Hostx64\arm` and causes "ARM vs ARM64" link errors.

- **Linux** from Windows: need a Linux C toolchain inside Docker. Run:
  - `cargo install cross`
  - Ensure **Docker Desktop** is installed and running.
  - `.\jade-lang\installers\linux\build-with-cross.ps1`  
  This runs `cross build --release --target x86_64-unknown-linux-gnu` and produces a Linux binary under `target/x86_64-unknown-linux-gnu/release/jade`.

## Installer matrix

| OS      | Arch    | Installer / method                    | Language recognition              |
|---------|---------|----------------------------------------|-----------------------------------|
| Windows | x86_64  | `.exe`, `.msi`, `.zip` or `install.ps1` | `.jdl` → jade, PATH, registry, icon |
| Windows | i686    | Same (cross-compile: `--target i686-pc-windows-msvc`) | Same                    |
| Windows | aarch64 | Same (cross-compile: `--target aarch64-pc-windows-msvc`) | Same                    |
| macOS   | x86_64  | `install.sh`, `.pkg`, `.dmg`          | UTI, “Open with Jade”             |
| macOS   | arm64   | Same                                  | Same                              |
| Linux   | x86_64  | `install.sh`, `.deb`, `.rpm`, `.AppImage` | MIME, `.desktop`, PATH        |
| Linux   | aarch64 | Same (e.g. `build-deb.sh 0.1.0 aarch64`) | Same                         |
| Android | aarch64 | `.tar.gz` (Termux)                    | Use in Termux shell               |

**Build all (Windows):** `.\jade-lang\installers\windows\build-all-windows.ps1` — x86_64, i686, aarch64. **Linux:** `./jade-lang/installers/linux/build-all-linux.sh`. **macOS:** `./jade-lang/installers/macos/build-all-macos.sh`.

## Contents

- **windows/** — Inno Setup (`.exe`), WiX (`.msi`), portable zip, `install.ps1`, file association, icon.
- **macos/** — `install.sh`, `build-pkg.sh` (`.pkg`), `build-dmg.sh` (`.dmg`); UTI for `.jdl`.
- **linux/** — `install.sh`, `build-deb.sh`, `build-rpm.sh`, `build-appimage.sh`; MIME, `.desktop`.
- **android/** — `build-termux.sh` (`.tar.gz` for Termux); see [android/README.md](android/README.md).
- **mime/** — Shared MIME info and IDE hints so IDEs recognize `.jdl` as Jade.

## Building installers

1. **Build the binary** in `jade-lang`: `cargo build --release`.
2. **Windows:** `windows/build-msi.ps1` (`.msi`), or `iscc windows/jade-setup.iss` (`.exe`), or `windows/build-portable.ps1` (`.zip`).
3. **macOS:** `macos/build-pkg.sh` (`.pkg`), `macos/build-dmg.sh` (`.dmg`); or `macos/install.sh`.
4. **Linux:** `linux/build-deb.sh` (`.deb`), `linux/build-rpm.sh` (`.rpm`, needs `fpm`), or `linux/build-appimage.sh` (`.AppImage`).
5. **Android:** `android/build-termux.sh` (requires `rustup target add aarch64-linux-android`).

Icon for installers and app: **`windows/icon/jade.ico`** (Windows). To make all `.jdl` files on your PC use this icon, run `windows/register-jade-icon.ps1`.
