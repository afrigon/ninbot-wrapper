use super::Palette;
use super::Piece;
use super::Weight;

#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub pieces: Vec<Piece>
}

impl Segment {
    pub fn new(pieces: Vec<Piece>) -> Self {
        Self { pieces }
    }

    pub fn text(text: impl Into<String>, color: Palette, weight: Weight) -> Self {
        Self::new(vec![Piece::text(text, color, weight)])
    }
}
