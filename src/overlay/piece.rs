use super::Palette;
use crate::ninjabrain::BoatState;

#[derive(Debug, Clone, PartialEq)]
pub enum Piece {
    Text { text: String, color: Palette },
    Hidden(String),
    Boat(BoatState)
}

impl Piece {
    pub fn text(text: impl Into<String>, color: Palette) -> Self {
        Self::Text {
            text: text.into(),
            color
        }
    }
}
