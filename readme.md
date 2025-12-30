# OrbitKeys

**OrbitKeys** is a native **COSMIC / Wayland** desktop application that displays **context-aware keyboard shortcuts** for the currently focused application.

It listens to COSMIC toplevel focus events and dynamically loads shortcut definitions from JSON files based on the active app’s `app_id`.

This project is intentionally **COSMIC-only**:


---

## Features

- Context-aware keyboard shortcut display
- Automatically updates when application focus changes
- Manual “Home / Root” mode for desktop-level shortcuts
- Minimal settings overlay
- JSON-based shortcut definitions
- Native Wayland + COSMIC protocols

---

## Requirements

- **COSMIC Desktop Environment**
- **Wayland**
- Rust toolchain (for building from source)

OrbitKeys depends on:
- `libcosmic`
- `COSMIC Wayland protocols`
- `ext-foreign-toplevel-list`
- `zcosmic_toplevel_info`

It will **not** work on GNOME, KDE, X11, or other compositors.

---

## Building

```bash
cargo build --release
```

---

## Installation Script and Manual (Local)

### Script

Install with the `./install` script. 
at root.


### Install binary Manual
```bash
mkdir -p ~/.local/bin
cp target/release/orbitkeys ~/.local/bin/
```

### Install desktop entry
```bash
mkdir -p ~/.local/share/applications
cp orbitkeys.desktop ~/.local/share/applications/xyz.fonzi.orbitkeys.desktop
```

### Install shortcut definitions
```bash
mkdir -p ~/.local/share/orbitkeys
cp -r shortcuts ~/.local/share/orbitkeys/
```

---

## Supported Applications

OrbitKeys loads shortcuts based on the focused application's `app_id`.  
Support is driven by JSON files in the `shortcuts/` directory.

Currently included shortcut sets:

### Web Browsers
- Chromium
- Firefox

### Editors & Writing
- Visual Studio Code
- Obsidian
- LibreOffice Writer
- LibreOffice Calc
- LibreOffice Impress
- LibreOffice Draw

### COSMIC Desktop
- COSMIC Desktop (global / root) on shortcut
- COSMIC Files
- COSMIC Terminal
- COSMIC Editor

### Design & Media
- GIMP
- Inkscape
- OBS Studio
- VLC

### Communication
- Discord
- Slack
- Thunderbird/BetterBird

### Other
- Spotify
- Goose (Desktop Goose)

---

Adding support for new applications is simple:
1. Create a new `<app_id>.json` file in the `shortcuts/` directory
2. Re-install
3. Restart OrbitKeys

Contributions are welcome.

## Shortcut Definitions

Shortcut files are stored at:

```
~/.local/share/orbitkeys/shortcuts/<app_id>.json
```

Example:

```json
{
  "shortcuts": [
    {
      "keys": "Ctrl+C",
      "description": "Copy selection",
      "category": "Editing"
    },
    {
      "keys": "Ctrl+V",
      "description": "Paste clipboard",
      "category": "Editing"
    }
  ]
}
```

Notes:
- `<app_id>` must match the Wayland `app_id`
- `category` is optional
- Files are loaded dynamically on focus change

---

## Usage

Launch OrbitKeys from the COSMIC launcher

---

## Project Status

OrbitKeys is early but functional.  
APIs and behavior may evolve alongside COSMIC.

Contributions and shortcut packs are welcome.

---

## Known Issues

- **Window close (❌) may not fully terminate the process**  
  In some cases, closing the window does not immediately exit the OrbitKeys process. This is
  currently believed to be caused by the background Wayland focus watcher keeping the process
  alive after the window is closed. This will be addressed in a future update.


## License

GNU General Public License v2.0 (GPL-2.0)
