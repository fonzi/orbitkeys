#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    // Add the combined variant for "Arrows"
    Arrows,
    Plus,
    Minus,
    Space,
}

impl KeyGlyph {
    /// Maps the Enum to the visual symbol
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
            KeyGlyph::Arrows => "↕↔", // Visual representation of the arrow cluster
            KeyGlyph::Plus => "+",
            KeyGlyph::Minus => "−",
            KeyGlyph::Space => "␣",
        }
    }

    /// Maps the Enum to the text label
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
            KeyGlyph::Arrows => "Arrows",
            KeyGlyph::Plus => "Plus",
            KeyGlyph::Minus => "Minus",
            KeyGlyph::Space => "Space",
        }
    }

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
        KeyGlyph::Arrows, // Added to legend
    ];
}