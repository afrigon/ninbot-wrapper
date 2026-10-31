use tiny_skia::Color;

const MAX_TURN_DEGREES: f32 = 180.0;
const HALFWAY: f32 = 0.5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Palette {
    Background,
    Text,
    Positive,
    Negative,
    HighCertainty,
    MediumCertainty,
    LowCertainty,
    Warning,
    Error,
    Turn(u8)
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
            Self::Warning => Color::from_rgba8(0xe8, 0xd6, 0x9e, 0xff),
            Self::Turn(degrees) => Self::turn(degrees)
        }
    }

    fn turn(degrees: u8) -> Color {
        let progress = (f32::from(degrees) / MAX_TURN_DEGREES).min(1.0);

        if progress <= HALFWAY {
            Self::mix(
                Self::HighCertainty.color(),
                Self::MediumCertainty.color(),
                progress / HALFWAY
            )
        } else {
            Self::mix(
                Self::MediumCertainty.color(),
                Self::LowCertainty.color(),
                (progress - HALFWAY) / HALFWAY
            )
        }
    }

    fn mix(from: Color, to: Color, amount: f32) -> Color {
        let channel = |from: f32, to: f32| from + (to - from) * amount;

        Color::from_rgba(
            channel(from.red(), to.red()),
            channel(from.green(), to.green()),
            channel(from.blue(), to.blue()),
            channel(from.alpha(), to.alpha())
        )
        .unwrap_or(to)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turn_runs_from_green_through_yellow_to_red() {
        assert_eq!(Palette::Turn(0).color(), Palette::HighCertainty.color());
        assert_eq!(Palette::Turn(90).color(), Palette::MediumCertainty.color());
        assert_eq!(Palette::Turn(180).color(), Palette::LowCertainty.color());
    }
}
