#!/usr/bin/env bash
set -euo pipefail

APP_ID="xyz.fonzi.orbitkeys"
BIN_NAME="orbitkeys"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

BIN_DIR="$HOME/.local/bin"
APP_DIR="$HOME/.local/share/applications"

# Where we install data files (JSON shortcuts)
DATA_DIR="$HOME/.local/share/orbitkeys"
SHORTCUTS_DST="$DATA_DIR/shortcuts"

BIN_PATH="$BIN_DIR/$BIN_NAME"
DESKTOP_SRC="$REPO_ROOT/orbitkeys.desktop"
DESKTOP_DST="$APP_DIR/${APP_ID}.desktop"

# CHANGE THIS if your folder name differs
SHORTCUTS_SRC="$REPO_ROOT/shortcuts"

echo "[orbitkeys] building (release)"
cargo build --release

echo "[orbitkeys] installing binary -> $BIN_PATH"
mkdir -p "$BIN_DIR"
cp "$REPO_ROOT/target/release/$BIN_NAME" "$BIN_PATH"
chmod +x "$BIN_PATH"

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
mkdir -p "$APP_DIR"
cp "$DESKTOP_SRC" "$DESKTOP_DST"

# Patch Exec to absolute path so COSMIC can launch it reliably
sed -i "s|^Exec=.*|Exec=$BIN_PATH|g" "$DESKTOP_DST"

# (Optional) patch TryExec if present
if grep -q '^TryExec=' "$DESKTOP_DST"; then
  sed -i "s|^TryExec=.*|TryExec=$BIN_PATH|g" "$DESKTOP_DST"
fi

update-desktop-database "$APP_DIR" 2>/dev/null || true

echo "[orbitkeys] done"
echo "Binary:     $BIN_PATH"
echo "Shortcuts:  $SHORTCUTS_DST"
echo "Desktop:    $DESKTOP_DST"
