#!/usr/bin/env bash
# Jade macOS install script
# Usage: ./install.sh [path-to-jade-binary]
# Run from repo: ./jade-lang/installers/macos/install.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_JADE="$(cd "$SCRIPT_DIR/../.." && pwd)"
DEFAULT_BINARY="$REPO_JADE/target/release/jade"
BINARY="${1:-$DEFAULT_BINARY}"
INSTALL_DIR="${JADE_INSTALL_DIR:-/usr/local/bin}"

if [[ ! -f "$BINARY" ]]; then
  echo "Jade binary not found at: $BINARY"
  echo "Build first: cd jade-lang && cargo build --release"
  exit 1
fi

# Prefer /usr/local/bin; if no write access, use ~/.local/bin
if [[ -w "$INSTALL_DIR" ]] || sudo -n true 2>/dev/null; then
  if [[ ! -w "$INSTALL_DIR" ]]; then
    sudo cp "$BINARY" "$INSTALL_DIR/jade"
    sudo chmod +x "$INSTALL_DIR/jade"
  else
    cp "$BINARY" "$INSTALL_DIR/jade"
    chmod +x "$INSTALL_DIR/jade"
  fi
  echo "Installed: $INSTALL_DIR/jade"
else
  LOCAL_BIN="$HOME/.local/bin"
  mkdir -p "$LOCAL_BIN"
  cp "$BINARY" "$LOCAL_BIN/jade"
  chmod +x "$LOCAL_BIN/jade"
  echo "Installed: $LOCAL_BIN/jade"
  if [[ ":$PATH:" != *":$LOCAL_BIN:"* ]]; then
    echo "Add to your shell profile: export PATH=\"\$HOME/.local/bin:\$PATH\""
  fi
fi

# Register UTI so macOS recognizes .jdl as "Jade source" (identity in Finder/Spotlight)
# This requires a plist; we install to Application Support so Launch Services can see it.
UTI_PLIST="$HOME/Library/Application Support/org.jade-lang.uti/Info.plist"
mkdir -p "$(dirname "$UTI_PLIST")"
cat > "$UTI_PLIST" << 'UTI_EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleIdentifier</key>
  <string>org.jade-lang.source</string>
  <key>CFBundleName</key>
  <string>Jade Source</string>
  <key>CFBundleVersion</key>
  <string>1.0</string>
  <key>CFBundleDocumentTypes</key>
  <array>
    <dict>
      <key>CFBundleTypeName</key>
      <string>Jade source file</string>
      <key>CFBundleTypeExtensions</key>
      <array><string>jdl</string></array>
      <key>CFBundleTypeRole</key>
      <string>Editor</string>
      <key>LSHandlerRank</key>
      <string>Owner</string>
    </dict>
  </array>
</dict>
</plist>
UTI_EOF
echo "Registered .jdl UTI for Finder/Spotlight (org.jade-lang.source)."
echo "To run .jdl files by double-click: Right-click file → Open With → Other → choose 'jade' or Terminal, then: jade \"\$file\""

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
