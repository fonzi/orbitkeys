#!/usr/bin/env bash
set -euo pipefail

APP_ID="xyz.fonzi.orbitkeys"
BIN_NAME="orbitkeys"

rm -f "$HOME/.local/bin/$BIN_NAME"
rm -f "$HOME/.local/share/applications/${APP_ID}.desktop"
rm -rf "$HOME/.local/share/orbitkeys"

update-desktop-database "$HOME/.local/share/applications" 2>/dev/null || true

echo "[orbitkeys] uninstalled"
