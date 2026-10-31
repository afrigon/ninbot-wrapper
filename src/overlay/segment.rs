use super::Palette;
use super::Piece;

#[derive(Debug, Clone, PartialEq)]
pub struct Segment {
    pub pieces: Vec<Piece>
}

impl Segment {
    pub fn new(pieces: Vec<Piece>) -> Self {
        Self { pieces }
    }

    pub fn text(text: impl Into<String>, color: Palette) -> Self {
        Self::new(vec![Piece::text(text, color)])
    }
}
