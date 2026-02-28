#!/usr/bin/env bash
# Build a .dmg disk image for Jade (macOS). Contains the .pkg installer.
# Requires: build-pkg.sh run first, or we build the pkg and then wrap it.
# Usage: ./installers/macos/build-dmg.sh [version]
# Output: dist/installers/macos/jade-<version>-macos-<arch>.dmg

set -e
VERSION="${1:-0.1.0}"
REQUEST_ARCH="${2:-$(uname -m)}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JADE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
OUT_DIR="$JADE_ROOT/dist/installers/macos"
ARCH="$REQUEST_ARCH"
[[ "$ARCH" == "x86_64" ]] || true
[[ "$ARCH" == "arm64" ]] || true

PKG="$OUT_DIR/jade-${VERSION}-macos-${ARCH}.pkg"
DMG="$OUT_DIR/jade-${VERSION}-macos-${ARCH}.dmg"

if [[ ! -f "$PKG" ]]; then
  echo "Building .pkg first for $ARCH..."
  "$SCRIPT_DIR/build-pkg.sh" "$VERSION" "$ARCH"
fi

mkdir -p "$OUT_DIR"
DMG_STAGE="$(mktemp -d)"
trap "rm -rf '$DMG_STAGE'" EXIT

# Create a minimal layout: copy pkg and optional README
cp "$PKG" "$DMG_STAGE/"

# Create read-only DMG
hdiutil create -volname "Jade $VERSION" -srcfolder "$DMG_STAGE" -ov -format UDZO "$DMG"

echo "Built: $DMG"
