#!/usr/bin/env bash
set -euo pipefail

APP_ID="xyz.fonzi.orbitkeys"
BIN_NAME="orbitkeys"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

BIN_DIR="$HOME/.local/bin"
APP_DIR="$HOME/.local/share/applications"
ICON_DIR="$HOME/.local/share/icons/hicolor"

# Where we install data files (JSON shortcuts)
DATA_DIR="$HOME/.local/share/orbitkeys"
SHORTCUTS_DST="$DATA_DIR/shortcuts"

BIN_PATH="$BIN_DIR/$BIN_NAME"
DESKTOP_SRC="$REPO_ROOT/orbitkeys.desktop"
DESKTOP_DST="$APP_DIR/${APP_ID}.desktop"

# CHANGE THIS if your folder name differs
SHORTCUTS_SRC="$REPO_ROOT/shortcuts"
ICONS_SRC="$REPO_ROOT/icons"

echo "[orbitkeys] building (release)"
cargo build --release

echo "[orbitkeys] installing binary -> $BIN_PATH"
mkdir -p "$BIN_DIR"
install -m755 "$REPO_ROOT/target/release/$BIN_NAME" "$BIN_PATH"

echo "[orbitkeys] installing shortcuts -> $SHORTCUTS_DST"
if [ ! -d "$SHORTCUTS_SRC" ]; then
  echo "ERROR: shortcuts dir not found: $SHORTCUTS_SRC"
  echo "Fix install.sh SHORTCUTS_SRC to match your repo folder."
  exit 1
fi
mkdir -p "$DATA_DIR"
rm -rf "$SHORTCUTS_DST"
cp -a "$SHORTCUTS_SRC" "$SHORTCUTS_DST"

echo "[orbitkeys] installing desktop entry -> $DESKTOP_DST"
if [ ! -f "$DESKTOP_SRC" ]; then
  echo "ERROR: desktop file not found: $DESKTOP_SRC"
  exit 1
fi
mkdir -p "$APP_DIR"
install -m644 "$DESKTOP_SRC" "$DESKTOP_DST"

# Install icons
echo "[orbitkeys] installing icons -> $ICON_DIR"
if [ -d "$ICONS_SRC" ]; then
  # Install each icon size to the appropriate directory
  for size in 16 22 24 32 48 64 128 256 512; do
    size_dir="$ICON_DIR/${size}x${size}/apps"
    mkdir -p "$size_dir"
    if [ -f "$ICONS_SRC/orbitkeys-${size}.png" ]; then
      install -m644 "$ICONS_SRC/orbitkeys-${size}.png" "$size_dir/xyz.fonzi.orbitkeys.png"
    fi
  done
else
  echo "WARNING: icons dir not found: $ICONS_SRC"
fi

# Optional (harmless) cache refreshes
update-desktop-database "$APP_DIR" 2>/dev/null || true

echo "[orbitkeys] done"
echo "Binary:     $BIN_PATH"
echo "Shortcuts:  $SHORTCUTS_DST"
echo "Desktop:    $DESKTOP_DST"
echo "Icons:      $ICON_DIR"
