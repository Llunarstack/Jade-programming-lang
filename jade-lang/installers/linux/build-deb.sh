#!/usr/bin/env bash
# Build a .deb package for Jade (Debian/Ubuntu).
# Usage: ./build-deb.sh [version] [arch]
# Arch: x86_64 (default), aarch64, i686, armv7. Output: dist/installers/linux/jade-<version>-linux-<arch>.deb

set -e
VERSION="${1:-0.1.0}"
REQUEST_ARCH="${2:-}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JADE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
OUT_DIR="$JADE_ROOT/dist/installers/linux"

# Map arch to Rust target and .deb architecture
if [[ -n "$REQUEST_ARCH" ]]; then
  case "$REQUEST_ARCH" in
    x86_64) RUST_TARGET="x86_64-unknown-linux-gnu"; ARCH="amd64" ;;
    aarch64|arm64) RUST_TARGET="aarch64-unknown-linux-gnu"; ARCH="arm64" ;;
    i686) RUST_TARGET="i686-unknown-linux-gnu"; ARCH="i386" ;;
    armv7) RUST_TARGET="armv7-unknown-linux-gnueabihf"; ARCH="armhf" ;;
    *) echo "Unknown arch: $REQUEST_ARCH"; exit 1 ;;
  esac
  if [[ "$RUST_TARGET" == "x86_64-unknown-linux-gnu" ]]; then
    BINARY="$JADE_ROOT/target/release/jade"
  else
    BINARY="$JADE_ROOT/target/$RUST_TARGET/release/jade"
  fi
else
  ARCH="$(dpkg --print-architecture 2>/dev/null || uname -m)"
  if [[ "$ARCH" == "x86_64" ]]; then ARCH="amd64"; fi
  if [[ "$ARCH" == "aarch64" ]]; then ARCH="arm64"; fi
  BINARY="$JADE_ROOT/target/release/jade"
fi

if [[ ! -f "$BINARY" ]]; then
  echo "Build first: cd jade-lang && cargo build --release${RUST_TARGET:+ --target $RUST_TARGET}"
  exit 1
fi

mkdir -p "$OUT_DIR"
BUILD_DIR="$(mktemp -d)"
trap "rm -rf '$BUILD_DIR'" EXIT
mkdir -p "$BUILD_DIR/DEBIAN" "$BUILD_DIR/usr/bin" "$BUILD_DIR/usr/share/mime/packages" "$BUILD_DIR/usr/share/applications"

cp "$BINARY" "$BUILD_DIR/usr/bin/jade"
cp "$SCRIPT_DIR/jade.xml" "$BUILD_DIR/usr/share/mime/packages/"
cat > "$BUILD_DIR/usr/share/applications/jade-run.desktop" << EOF
[Desktop Entry]
Type=Application
Name=Run with Jade
Comment=Run .jdl file with Jade interpreter
Exec=/usr/bin/jade %f
Icon=text-x-script
Terminal=true
MimeType=text/x-jdl;application/x-jdl
Categories=Development;
EOF

cat > "$BUILD_DIR/DEBIAN/control" << EOF
Package: jade-lang
Version: $VERSION
Section: devel
Priority: optional
Architecture: $ARCH
Maintainer: Jade Language Team
Description: Jade programming language (interpreter, JIT, AOT)
 Run: jade file.jdl
 Build: jade build file.jdl -o app
EOF
echo "#!/bin/sh" > "$BUILD_DIR/DEBIAN/postinst"
echo "if command -v update-mime-database >/dev/null 2>&1; then update-mime-database /usr/share/mime; fi" >> "$BUILD_DIR/DEBIAN/postinst"
chmod 755 "$BUILD_DIR/DEBIAN/postinst"

dpkg-deb --build -Zgzip "$BUILD_DIR" "$OUT_DIR/jade-${VERSION}-linux-${ARCH}.deb" 2>/dev/null || dpkg-deb --build "$BUILD_DIR" "$OUT_DIR/jade-${VERSION}-linux-${ARCH}.deb"
echo "Built: $OUT_DIR/jade-${VERSION}-linux-${ARCH}.deb"
