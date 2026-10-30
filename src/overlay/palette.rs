use tiny_skia::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Palette {
    Background,
    Text,
    Positive,
    Negative,
    HighCertainty,
    MediumCertainty,
    LowCertainty,
    Meta,
    Warning,
    Error
}

impl Palette {
    pub fn color(self) -> Color {
        match self {
            Self::Background => Color::from_rgba8(90, 90, 90, 102),
            Self::Text => Color::from_rgba8(0xe0, 0xe0, 0xe0, 0xff),
            Self::Positive | Self::HighCertainty => Color::from_rgba8(0x9e, 0xe8, 0x9e, 0xff),
            Self::Negative | Self::LowCertainty | Self::Error => {
                Color::from_rgba8(0xff, 0x9e, 0x9e, 0xff)
            }
            Self::MediumCertainty => Color::from_rgba8(0xd6, 0xe8, 0x9e, 0xff),
            Self::Meta => Color::from_rgba8(0x9a, 0x9a, 0x9a, 0xff),
            Self::Warning => Color::from_rgba8(0xe8, 0xd6, 0x9e, 0xff)
        }
    }
}
