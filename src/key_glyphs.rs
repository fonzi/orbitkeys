#[derive(Debug, Clone, Copy)]
pub enum KeyGlyph {
    Ctrl,
    Shift,
    Alt,
    Super,

    Tab,
    Enter,
    Escape,
    Backspace,

    Left,
    Right,
    Up,
    Down,

    Plus,
    Minus,

    Space,
}

impl KeyGlyph {
    pub fn as_str(self) -> &'static str {
        match self {
            KeyGlyph::Ctrl => "⌃",
            KeyGlyph::Shift => "⇧",
            KeyGlyph::Alt => "⎇",
            KeyGlyph::Super => "⌘",

            KeyGlyph::Tab => "⇥",
            KeyGlyph::Enter => "↵",
            KeyGlyph::Escape => "⎋",
            KeyGlyph::Backspace => "⌫",

            KeyGlyph::Left => "←",
            KeyGlyph::Right => "→",
            KeyGlyph::Up => "↑",
            KeyGlyph::Down => "↓",

            KeyGlyph::Plus => "+",
            KeyGlyph::Minus => "−",

            KeyGlyph::Space => "␣",
        }
    }
}
