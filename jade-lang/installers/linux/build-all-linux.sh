#!/usr/bin/env bash
# Build Jade for all Linux architectures: x86_64, aarch64 (optional: i686, armv7).
# Produces: .deb, .rpm (if fpm installed), and optionally AppImage for x86_64.
# Usage: ./build-all-linux.sh [version]. Requires: rustup target add aarch64-unknown-linux-gnu (etc.)

set -e
VERSION="${1:-0.1.0}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JADE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

ARCHS=("x86_64" "aarch64")
for arch in "${ARCHS[@]}"; do
  echo "=== Building Jade for $arch ==="
  if [[ "$arch" == "x86_64" ]]; then
    cargo build --release
    BINARY="$JADE_ROOT/target/release/jade"
  else
    rust_target="aarch64-unknown-linux-gnu"
    cargo build --release --target "$rust_target"
    BINARY="$JADE_ROOT/target/$rust_target/release/jade"
  fi
  [[ -f "$BINARY" ]] || { echo "Build failed for $arch"; exit 1; }

  "$SCRIPT_DIR/build-deb.sh" "$VERSION" "$arch" || true
  "$SCRIPT_DIR/build-rpm.sh" "$VERSION" "$arch" || true
done
# AppImage typically x86_64 only
"$SCRIPT_DIR/build-appimage.sh" "$VERSION" 2>/dev/null || true

OUT_DIR="$JADE_ROOT/dist/installers/linux"
echo "Done. Outputs in: $OUT_DIR"
ls -la "$OUT_DIR" 2>/dev/null || true
