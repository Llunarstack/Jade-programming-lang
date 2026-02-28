#!/usr/bin/env bash
# Build a .rpm package for Jade (Fedora/RHEL/openSUSE).
# Usage: ./build-rpm.sh [version] [arch]. Arch: x86_64, aarch64, i686 (default: current).
set -e
VERSION="${1:-0.1.0}"
REQUEST_ARCH="${2:-}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
JADE_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
OUT_DIR="$JADE_ROOT/dist/installers/linux"

if [[ -n "$REQUEST_ARCH" ]]; then
  case "$REQUEST_ARCH" in
    x86_64) RUST_TARGET="x86_64-unknown-linux-gnu"; ARCH="x86_64" ;;
    aarch64) RUST_TARGET="aarch64-unknown-linux-gnu"; ARCH="aarch64" ;;
    i686) RUST_TARGET="i686-unknown-linux-gnu"; ARCH="i686" ;;
    *) echo "Unknown arch: $REQUEST_ARCH"; exit 1 ;;
  esac
  BINARY="$JADE_ROOT/target/$RUST_TARGET/release/jade"
  [[ "$REQUEST_ARCH" == "x86_64" ]] && BINARY="$JADE_ROOT/target/release/jade"
else
  ARCH="$(uname -m)"
  BINARY="$JADE_ROOT/target/release/jade"
fi
if [[ ! -f "$BINARY" ]]; then
  echo "Build first: cd jade-lang && cargo build --release${RUST_TARGET:+ --target $RUST_TARGET}"
  exit 1
fi

if ! command -v fpm &>/dev/null; then
  echo "fpm not found. Install with: gem install fpm  (or: dnf install fpm)"
  exit 1
fi

mkdir -p "$OUT_DIR"
STAGE="$(mktemp -d)"
trap "rm -rf '$STAGE'" EXIT
mkdir -p "$STAGE/usr/bin" "$STAGE/usr/share/mime/packages" "$STAGE/usr/share/applications"

cp "$BINARY" "$STAGE/usr/bin/jade"
cp "$SCRIPT_DIR/jade.xml" "$STAGE/usr/share/mime/packages/"
cat > "$STAGE/usr/share/applications/jade-run.desktop" << EOF
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

fpm -s dir -t rpm -n jade-lang -v "$VERSION" --license MIT \
  -C "$STAGE" \
  -p "$OUT_DIR/jade-${VERSION}-linux-${ARCH}.rpm" \
  --rpm-os linux \
  --description "Jade programming language (interpreter, JIT, AOT). Run: jade file.jdl" \
  usr/bin/jade usr/share/mime/packages/jade.xml usr/share/applications/jade-run.desktop

echo "Built: $OUT_DIR/jade-${VERSION}-linux-${ARCH}.rpm"
