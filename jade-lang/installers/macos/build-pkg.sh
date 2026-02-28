#!/usr/bin/env bash
# Build a .pkg installer for Jade (macOS).
# Usage: ./build-pkg.sh [version] [arch]. Arch: x86_64, arm64 (default: current).
set -e
VERSION="${1:-0.1.0}"
REQUEST_ARCH="${2:-}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JADE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
OUT_DIR="$JADE_ROOT/dist/installers/macos"

if [[ -n "$REQUEST_ARCH" ]]; then
  ARCH="$REQUEST_ARCH"
  [[ "$ARCH" == "arm64" ]] && ARCH="arm64"
  if [[ "$ARCH" == "x86_64" ]]; then
    BINARY="$JADE_ROOT/target/x86_64-apple-darwin/release/jade"
  else
    BINARY="$JADE_ROOT/target/aarch64-apple-darwin/release/jade"
    ARCH="arm64"
  fi
else
  ARCH="$(uname -m)"
  [[ "$ARCH" == "arm64" ]] && true
  BINARY="$JADE_ROOT/target/release/jade"
fi

if [[ ! -f "$BINARY" ]]; then
  echo "Build first: cd jade-lang && cargo build --release [--target x86_64-apple-darwin|aarch64-apple-darwin]"
  exit 1
fi

mkdir -p "$OUT_DIR"
STAGE="$(mktemp -d)"
trap "rm -rf '$STAGE'" EXIT
mkdir -p "$STAGE/usr/local/bin"
cp "$BINARY" "$STAGE/usr/local/bin/jade"
chmod +x "$STAGE/usr/local/bin/jade"

PKG_ID="org.jade-lang.jade"
COMPONENT_PKG="$STAGE/jade-component.pkg"
FINAL_PKG="$OUT_DIR/jade-${VERSION}-macos-${ARCH}.pkg"

pkgbuild --root "$STAGE" \
  --identifier "$PKG_ID" \
  --version "$VERSION" \
  --install-location "/" \
  "$COMPONENT_PKG"

productbuild --package "$COMPONENT_PKG" \
  --identifier "$PKG_ID" \
  --version "$VERSION" \
  "$FINAL_PKG"

echo "Built: $FINAL_PKG"
