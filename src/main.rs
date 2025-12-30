mod focus_watcher;
mod shortcut_resolver;
mod ui;
mod key_glyphs;
mod key_format;

use anyhow::Result;
use std::path::PathBuf;

use shortcut_resolver::ShortcutResolver;

fn main() -> Result<()> {
    let shortcuts_path = find_shortcuts_directory();
    let resolver = ShortcutResolver::new(shortcuts_path)?;

    let settings = cosmic::app::Settings::default()
        .is_daemon(false)
        .exit_on_close(true);

    cosmic::app::run::<ui::OrbitKeysUi>(settings, resolver)?;
    Ok(())
}

fn find_shortcuts_directory() -> PathBuf {
    if let Ok(dir) = std::env::var("XDG_DATA_HOME") {
        let p = PathBuf::from(dir).join("orbitkeys/shortcuts");
        if p.exists() {
            return p;
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        let p = PathBuf::from(home).join(".local/share/orbitkeys/shortcuts");
        if p.exists() {
            return p;
        }
    }

    PathBuf::from("shortcuts")
}
