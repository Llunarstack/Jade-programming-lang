#!/usr/bin/env bash
# Build Jade for both macOS architectures: x86_64 and arm64 (Apple Silicon).
# Produces: .pkg and .dmg for each. Run on macOS (cross-compile or native).
# Usage: ./build-all-macos.sh [version]

set -e
VERSION="${1:-0.1.0}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JADE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

for arch in x86_64 arm64; do
  echo "=== Building Jade for $arch ==="
  if [[ "$arch" == "x86_64" ]]; then
    cargo build --release --target x86_64-apple-darwin
  else
    cargo build --release --target aarch64-apple-darwin
  fi
  "$SCRIPT_DIR/build-pkg.sh" "$VERSION" "$arch" || true
  "$SCRIPT_DIR/build-dmg.sh" "$VERSION" "$arch" || true
done
echo "Done. Outputs in: $JADE_ROOT/dist/installers/macos"
ls -la "$JADE_ROOT/dist/installers/macos" 2>/dev/null || true
