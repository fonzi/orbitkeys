mod focus_watcher;
mod shortcut_resolver;
mod ui;
mod key_glyphs;
mod key_format;


use anyhow::Result;
use shortcut_resolver::ShortcutResolver;

fn main() -> Result<()> {
    let resolver = ShortcutResolver::new("./shortcuts")?;
    cosmic::app::run::<ui::OrbitKeysUi>(cosmic::app::Settings::default(), resolver)?;
    Ok(())
}
