#!/usr/bin/env bash
# Build Jade for Android (Termux). Produces a tarball with the jade binary.
# Requires: rustup target add aarch64-linux-android (or armv7-linux-androideabi), and NDK/standalone toolchain if needed.
# Usage: ./installers/android/build-termux.sh [version]
# Output: dist/installers/android/jade-<version>-android-<arch>.tar.gz

set -e
VERSION="${1:-0.1.0}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JADE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
OUT_DIR="$JADE_ROOT/dist/installers/android"

# Default: aarch64 (most current Android devices)
TARGET="${JADE_ANDROID_TARGET:-aarch64-linux-android}"
if ! rustup target list --installed 2>/dev/null | grep -q "^$TARGET$"; then
  echo "Add Android target: rustup target add $TARGET"
  exit 1
fi

ARCH="${TARGET%%-*}"
if [[ "$TARGET" == armv7* ]]; then ARCH="armv7"; fi

mkdir -p "$OUT_DIR"
cd "$JADE_ROOT"
cargo build --release --target "$TARGET" 2>/dev/null || {
  echo "Cross-compile may need NDK. Try: cargo install cross && cross build --release --target $TARGET"
  exit 1
}

BINARY="$JADE_ROOT/target/$TARGET/release/jade"
if [[ ! -f "$BINARY" ]]; then
  echo "Build failed for $TARGET"
  exit 1
fi

STAGE="$(mktemp -d)"
trap "rm -rf '$STAGE'" EXIT
cp "$BINARY" "$STAGE/jade"
chmod +x "$STAGE/jade"
tar czf "$OUT_DIR/jade-${VERSION}-android-${ARCH}.tar.gz" -C "$STAGE" jade
echo "Built: $OUT_DIR/jade-${VERSION}-android-${ARCH}.tar.gz"
