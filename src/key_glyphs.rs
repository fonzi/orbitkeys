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

    pub fn label(self) -> &'static str {
        match self {
            KeyGlyph::Ctrl => "Ctrl",
            KeyGlyph::Shift => "Shift",
            KeyGlyph::Alt => "Alt",
            KeyGlyph::Super => "Super",

            KeyGlyph::Tab => "Tab",
            KeyGlyph::Enter => "Enter",
            KeyGlyph::Escape => "Esc",
            KeyGlyph::Backspace => "Backspace",

            KeyGlyph::Left => "Left",
            KeyGlyph::Right => "Right",
            KeyGlyph::Up => "Up",
            KeyGlyph::Down => "Down",

            KeyGlyph::Plus => "Plus",
            KeyGlyph::Minus => "Minus",

            KeyGlyph::Space => "Space",
        }
    }

    // Legend is a *subset* of glyphs, order controlled here.
    pub const LEGEND: &'static [KeyGlyph] = &[
        KeyGlyph::Super,
        KeyGlyph::Ctrl,
        KeyGlyph::Alt,
        KeyGlyph::Shift,
        KeyGlyph::Tab,
        KeyGlyph::Enter,
        KeyGlyph::Escape,
        KeyGlyph::Backspace,
        KeyGlyph::Space,
    ];
}
