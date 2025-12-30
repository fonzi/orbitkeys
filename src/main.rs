mod focus_watcher;
mod shortcut_resolver;
mod ui;
mod key_glyphs;
mod key_format;

use anyhow::Result;
use shortcut_resolver::ShortcutResolver;
use std::path::PathBuf;

fn main() -> Result<()> {
    let shortcuts_path = find_shortcuts_directory();
    let resolver = ShortcutResolver::new(shortcuts_path)?;
    cosmic::app::run::<ui::OrbitKeysUi>(cosmic::app::Settings::default(), resolver)?;
    Ok(())
}

fn find_shortcuts_directory() -> PathBuf {
    // Try installed location first
    if let Some(home) = std::env::var("HOME").ok() {
        let installed_path = PathBuf::from(home).join(".local/share/orbitkeys/shortcuts");
        if installed_path.exists() {
            return installed_path;
        }
    }
    
    // Fall back to development location
    PathBuf::from("./shortcuts")
}
