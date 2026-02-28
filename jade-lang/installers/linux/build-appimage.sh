#!/usr/bin/env bash
# Build an AppImage for Jade (portable Linux binary bundle).
# Requires: cargo build --release, and either appimagetool (from AppImageKit) or mksquashfs + desktop-file-validate.
# Usage: ./installers/linux/build-appimage.sh [version]
# Output: dist/installers/linux/jade-<version>-linux-x86_64.AppImage

set -e
VERSION="${1:-0.1.0}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JADE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
OUT_DIR="$JADE_ROOT/dist/installers/linux"
BINARY="$JADE_ROOT/target/release/jade"
ARCH="$(uname -m)"

if [[ ! -f "$BINARY" ]]; then
  echo "Build first: cd jade-lang && cargo build --release"
  exit 1
fi

mkdir -p "$OUT_DIR"
APP_DIR="$(mktemp -d)"
trap "rm -rf '$APP_DIR'" EXIT

mkdir -p "$APP_DIR/usr/bin" "$APP_DIR/usr/share/mime/packages" "$APP_DIR/usr/share/applications"
cp "$BINARY" "$APP_DIR/usr/bin/jade"
cp "$SCRIPT_DIR/jade.xml" "$APP_DIR/usr/share/mime/packages/"
cat > "$APP_DIR/usr/share/applications/jade.desktop" << EOF
[Desktop Entry]
Type=Application
Name=Jade
Comment=Jade programming language
Exec=jade repl
Icon=jade
Terminal=true
Categories=Development;
EOF

# AppRun script at AppDir root
cat > "$APP_DIR/AppRun" << 'APPRUN'
#!/bin/sh
HERE="$(dirname "$(readlink -f "$0")")"
PATH="$HERE/usr/bin:$PATH"
exec "$HERE/usr/bin/jade" "$@"
APPRUN
chmod +x "$APP_DIR/AppRun"

if command -v appimagetool &>/dev/null; then
  appimagetool "$APP_DIR" "$OUT_DIR/jade-${VERSION}-linux-${ARCH}.AppImage"
  echo "Built: $OUT_DIR/jade-${VERSION}-linux-${ARCH}.AppImage"
  exit 0
fi

# Fallback: create a squashfs and wrap with type-2 AppImage runtime (download if needed)
if command -v mksquashfs &>/dev/null; then
  RUNTIME_URL="https://github.com/AppImage/AppImageKit/releases/download/continuous/runtime-x86_64"
  RUNTIME="$APP_DIR/runtime"
  if [[ "$ARCH" == "x86_64" ]]; then
    ( cd "$APP_DIR" && curl -sL -o runtime "$RUNTIME_URL" && chmod +x runtime ) || true
  fi
  if [[ -x "$APP_DIR/runtime" ]]; then
    mksquashfs "$APP_DIR" "$OUT_DIR/jade-${VERSION}-linux-${ARCH}.squashfs" -root-owned -noappend
    cat "$APP_DIR/runtime" "$OUT_DIR/jade-${VERSION}-linux-${ARCH}.squashfs" > "$OUT_DIR/jade-${VERSION}-linux-${ARCH}.AppImage"
    chmod +x "$OUT_DIR/jade-${VERSION}-linux-${ARCH}.AppImage"
    rm -f "$OUT_DIR/jade-${VERSION}-linux-${ARCH}.squashfs"
    echo "Built: $OUT_DIR/jade-${VERSION}-linux-${ARCH}.AppImage"
  else
    echo "Install appimagetool from AppImageKit, or add runtime manually. Creating squashfs only."
    mksquashfs "$APP_DIR" "$OUT_DIR/jade-${VERSION}-linux-${ARCH}.squashfs" -root-owned -noappend
    echo "Built: $OUT_DIR/jade-${VERSION}-linux-${ARCH}.squashfs (convert to AppImage with appimagetool)"
  fi
else
  echo "Install appimagetool (AppImageKit) or mksquashfs to build AppImage."
  exit 1
fi
