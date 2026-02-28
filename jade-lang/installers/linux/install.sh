#!/usr/bin/env bash
# Jade Linux install script (x86_64 / aarch64)
# Usage: ./install.sh [path-to-jade-binary]
# Run from repo: ./jade-lang/installers/linux/install.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_JADE="$(cd "$SCRIPT_DIR/../.." && pwd)"
DEFAULT_BINARY="$REPO_JADE/target/release/jade"
BINARY="${1:-$DEFAULT_BINARY}"
INSTALL_DIR="${JADE_INSTALL_DIR:-$HOME/.local/bin}"
MIME_DIR="$HOME/.local/share/mime"
APPS_DIR="$HOME/.local/share/applications"

if [[ ! -f "$BINARY" ]]; then
  echo "Jade binary not found at: $BINARY"
  echo "Build first: cd jade-lang && cargo build --release"
  exit 1
fi

mkdir -p "$INSTALL_DIR"
cp "$BINARY" "$INSTALL_DIR/jade"
chmod +x "$INSTALL_DIR/jade"
echo "Installed: $INSTALL_DIR/jade"

if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
  echo "Add to your shell profile: export PATH=\"\$HOME/.local/bin:\$PATH\""
fi

# Freedesktop MIME type so IDEs and file managers recognize .jdl
mkdir -p "$MIME_DIR/packages"
cat > "$MIME_DIR/packages/jade.xml" << 'MIME_EOF'
<?xml version="1.0" encoding="UTF-8"?>
<mime-info xmlns="http://www.freedesktop.org/standards/shared-mime-info">
  <mime-type type="text/x-jdl">
    <comment>Jade source code</comment>
    <comment xml:lang="en">Jade source code</comment>
    <glob pattern="*.jdl"/>
    <alias type="application/x-jdl"/>
  </mime-type>
</mime-info>
MIME_EOF
if command -v update-mime-database &>/dev/null; then
  update-mime-database "$MIME_DIR"
  echo "Registered MIME type: text/x-jdl"
fi

# Freedesktop MIME icon so .jdl files and IDEs show Jade logo (no extension needed)
ICON_SRC="$REPO_JADE/installers/windows/icon/jade.png"
if [[ -f "$ICON_SRC" ]]; then
  ICON_DIR="$HOME/.local/share/icons/hicolor"
  for size in 16 22 32 48 256; do
    mkdir -p "$ICON_DIR/${size}x${size}/mimetypes"
    cp "$ICON_SRC" "$ICON_DIR/${size}x${size}/mimetypes/text-x-jdl.png" 2>/dev/null || true
  done
  if command -v gtk-update-icon-cache &>/dev/null; then
    gtk-update-icon-cache -f -t "$ICON_DIR" 2>/dev/null || true
  fi
  echo "Installed Jade MIME icon (text-x-jdl) for file managers and IDEs"
fi

# "Open with Jade" in file managers
mkdir -p "$APPS_DIR"
JADE_CMD="jade"
which jade &>/dev/null || JADE_CMD="$INSTALL_DIR/jade"
cat > "$APPS_DIR/jade-run.desktop" << DESKTOP_EOF
[Desktop Entry]
Type=Application
Name=Run with Jade
Comment=Run .jdl file with Jade interpreter
Exec=$JADE_CMD %f
Icon=text-x-jdl
Terminal=true
MimeType=text/x-jdl;application/x-jdl
Categories=Development;
DESKTOP_EOF
echo "Desktop entry: $APPS_DIR/jade-run.desktop (Open with Jade)"

# Install Jade extension for VS Code and Cursor (syntax, run from buffer, autosave)
install_ide_extension() {
  local ext_src="$REPO_JADE/installers/ide/vscode-snippet"
  local ext_name="jade-language-0.2.0"
  [[ -d "$ext_src" ]] || return 0
  for dir in "$HOME/.vscode/extensions" "$HOME/.cursor/extensions"; do
    if [[ -d "$(dirname "$dir")" ]] || mkdir -p "$(dirname "$dir")" 2>/dev/null; then
      mkdir -p "$dir/$ext_name"
      cp -R "$ext_src"/* "$dir/$ext_name/"
      echo "Installed Jade extension into $dir/$ext_name"
    fi
  done
}
install_ide_extension

jade --version 2>/dev/null || true
echo "Done. Run: jade --help"
