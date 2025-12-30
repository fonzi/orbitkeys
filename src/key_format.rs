use crate::key_glyphs::KeyGlyph;

pub fn pretty_keys(raw: &str) -> String {
    let mut s = raw.to_string();

    let map = [
        // Modifiers with pluses
        ("Ctrl+", KeyGlyph::Ctrl),
        ("Shift+", KeyGlyph::Shift),
        ("Alt+", KeyGlyph::Alt),
        ("Super+", KeyGlyph::Super),

        // Standalone keys
        ("Tab", KeyGlyph::Tab),
        ("Enter", KeyGlyph::Enter),
        ("Esc", KeyGlyph::Escape),
        ("Escape", KeyGlyph::Escape),
        ("Backspace", KeyGlyph::Backspace),

        // Directionals
        ("Arrows", KeyGlyph::Arrows), // THE FIX: Plural first
        ("Arrow", KeyGlyph::Arrows),  // Catch singular too
        ("Left", KeyGlyph::Left),
        ("Right", KeyGlyph::Right),
        ("Up", KeyGlyph::Up),
        ("Down", KeyGlyph::Down),

        ("Plus", KeyGlyph::Plus),
        ("Minus", KeyGlyph::Minus),
        ("Space", KeyGlyph::Space),
        
        // Final fallback for modifiers without a trailing plus
        ("Ctrl", KeyGlyph::Ctrl),
        ("Shift", KeyGlyph::Shift),
        ("Alt", KeyGlyph::Alt),
        ("Super", KeyGlyph::Super),
    ];

    for (needle, glyph) in map {
        // Use glyph.as_str() for the icon, and maybe add a space 
        // if you want the icons to be separated.
        s = s.replace(needle, glyph.as_str());
    }

    s
}