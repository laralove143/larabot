#[allow(clippy::unreadable_literal)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    Blurple = 0x5865F2,
    Green = 0x57F287,
    Yellow = 0xFEE75C,
    Fuchsia = 0xEB459E,
    Red = 0xED4245,
    White = 0xFFFFFF,
    Black = 0x000000,
}

impl From<Color> for u32 {
    #[allow(clippy::as_conversions)]
    fn from(color: Color) -> Self {
        color as Self
    }
}
