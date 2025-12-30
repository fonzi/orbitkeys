use crate::key_glyphs::KeyGlyph;

pub fn pretty_keys(raw: &str) -> String {
    let mut s = raw.to_string();

    let map = [
        ("Ctrl+", KeyGlyph::Ctrl),
        ("Shift+", KeyGlyph::Shift),
        ("Alt+", KeyGlyph::Alt),
        ("Super+", KeyGlyph::Super),

        ("Tab", KeyGlyph::Tab),
        ("Enter", KeyGlyph::Enter),
        ("Esc", KeyGlyph::Escape),
        ("Escape", KeyGlyph::Escape),
        ("Backspace", KeyGlyph::Backspace),

        ("Left", KeyGlyph::Left),
        ("Right", KeyGlyph::Right),
        ("Up", KeyGlyph::Up),
        ("Down", KeyGlyph::Down),

        ("Plus", KeyGlyph::Plus),
        ("Minus", KeyGlyph::Minus),

        ("Space", KeyGlyph::Space),
    ];

    for (needle, glyph) in map {
        s = s.replace(needle, glyph.as_str());
    }

    s
}
