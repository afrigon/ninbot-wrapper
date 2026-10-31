use anyhow::Context;
use anyhow::Result;
use tiny_skia::Paint;
use tiny_skia::Pixmap;
use tiny_skia::PixmapPaint;
use tiny_skia::Rect;
use tiny_skia::Transform;

use super::Glyph;
use super::Line;
use super::Palette;
use super::Piece;
use crate::ninjabrain::BoatState;

const SCALE: u32 = 3;
const GLYPH_SPACING: u32 = 1;
const SEGMENT_GAP: u32 = 12;
const PADDING_TOP: u32 = 3;
const PADDING_RIGHT: u32 = 3;
const PADDING_BOTTOM: u32 = 0;
const PADDING_LEFT: u32 = 3;
const CONTENT_HEIGHT: u32 = Glyph::HEIGHT * SCALE;
const EMPTY_SIZE: u32 = 1;

pub struct Renderer {
    boat_none: Pixmap,
    boat_measuring: Pixmap,
    boat_valid: Pixmap,
    boat_error: Pixmap
}

impl Renderer {
    pub fn new() -> Result<Self> {
        Ok(Self {
            boat_none: Pixmap::decode_png(include_bytes!("../../assets/boat/none.png"))?,
            boat_measuring: Pixmap::decode_png(include_bytes!("../../assets/boat/measuring.png"))?,
            boat_valid: Pixmap::decode_png(include_bytes!("../../assets/boat/valid.png"))?,
            boat_error: Pixmap::decode_png(include_bytes!("../../assets/boat/error.png"))?
        })
    }

    pub fn render(&self, line: &Line) -> Result<Vec<u8>> {
        if line.is_empty() {
            let pixmap = Pixmap::new(EMPTY_SIZE, EMPTY_SIZE).context("empty frame")?;
            return Ok(pixmap.encode_png()?);
        }

        let pieces = line.segments.iter().map(|segment| {
            segment
                .pieces
                .iter()
                .map(|piece| self.width(piece))
                .sum::<u32>()
        });
        let content_width = pieces.sum::<u32>() + SEGMENT_GAP * (line.segments.len() as u32 - 1);
        let width = PADDING_LEFT + content_width + PADDING_RIGHT;
        let height = PADDING_TOP + CONTENT_HEIGHT + PADDING_BOTTOM;
        let mut pixmap = Pixmap::new(width, height).context("frame size")?;
        pixmap.fill(Palette::Background.color());

        let mut x = PADDING_LEFT;

        for segment in &line.segments {
            for piece in &segment.pieces {
                self.draw(&mut pixmap, piece, x, PADDING_TOP);
                x += self.width(piece);
            }

            x += SEGMENT_GAP;
        }

        Ok(pixmap.encode_png()?)
    }

    fn width(&self, piece: &Piece) -> u32 {
        match piece {
            Piece::Text { text, .. } => text
                .chars()
                .map(|character| Self::advance(Glyph::of(character)) * SCALE)
                .sum(),
            Piece::Boat(state) => self.boat(*state).width()
        }
    }

    fn draw(&self, pixmap: &mut Pixmap, piece: &Piece, x: u32, y: u32) {
        match piece {
            Piece::Text { text, color } => {
                let mut paint = Paint::default();
                paint.set_color(color.color());
                paint.anti_alias = false;

                let mut x = x;

                for character in text.chars() {
                    let glyph = Glyph::of(character);
                    Self::draw_glyph(pixmap, glyph, &paint, x, y);
                    x += Self::advance(glyph) * SCALE;
                }
            }
            Piece::Boat(state) => {
                let icon = self.boat(*state);
                let y = y + (CONTENT_HEIGHT - icon.height()) / 2;

                pixmap.draw_pixmap(
                    x as i32,
                    y as i32,
                    icon.as_ref(),
                    &PixmapPaint::default(),
                    Transform::identity(),
                    None
                );
            }
        }
    }

    fn draw_glyph(pixmap: &mut Pixmap, glyph: &Glyph, paint: &Paint, x: u32, y: u32) {
        for (column, row) in glyph.lit_pixels() {
            let Some(pixel) = Rect::from_xywh(
                (x + column * SCALE) as f32,
                (y + row * SCALE) as f32,
                SCALE as f32,
                SCALE as f32
            ) else {
                continue;
            };

            pixmap.fill_rect(pixel, paint, Transform::identity(), None);
        }
    }

    fn advance(glyph: &Glyph) -> u32 {
        glyph.width() + GLYPH_SPACING
    }

    fn boat(&self, state: BoatState) -> &Pixmap {
        match state {
            BoatState::None => &self.boat_none,
            BoatState::Measuring => &self.boat_measuring,
            BoatState::Valid => &self.boat_valid,
            BoatState::Error => &self.boat_error
        }
    }
}
