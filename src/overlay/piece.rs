use super::Palette;
use super::Weight;
use crate::ninjabrain::BoatState;

#[derive(Debug, Clone, PartialEq)]
pub enum Piece {
    Text {
        text: String,
        color: Palette,
        weight: Weight
    },
    Boat(BoatState)
}

impl Piece {
    pub fn text(text: impl Into<String>, color: Palette, weight: Weight) -> Self {
        Self::Text {
            text: text.into(),
            color,
            weight
        }
    }
}
