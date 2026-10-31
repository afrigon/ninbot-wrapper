const ROWS: usize = 8;
const WIDTH: u32 = 5;
const LIT: u8 = b'#';

#[derive(Debug, Clone, Copy)]
pub struct Glyph {
    rows: [&'static str; ROWS]
}

impl Glyph {
    pub const HEIGHT: u32 = ROWS as u32;

    const fn new(rows: [&'static str; ROWS]) -> Self {
        Self { rows }
    }

    pub fn of(character: char) -> &'static Glyph {
        match character {
            '0' => &DIGIT_0,
            '1' => &DIGIT_1,
            '2' => &DIGIT_2,
            '3' => &DIGIT_3,
            '4' => &DIGIT_4,
            '5' => &DIGIT_5,
            '6' => &DIGIT_6,
            '7' => &DIGIT_7,
            '8' => &DIGIT_8,
            '9' => &DIGIT_9,
            'a' => &A,
            'b' => &B,
            'c' => &C,
            'd' => &D,
            'e' => &E,
            'f' => &F,
            'g' => &G,
            'h' => &H,
            'i' => &I,
            'j' => &J,
            'k' => &K,
            'l' => &L,
            'm' => &M,
            'n' => &N,
            'o' => &O,
            'p' => &P,
            'q' => &Q,
            'r' => &R,
            's' => &S,
            't' => &T,
            'u' => &U,
            'v' => &V,
            'w' => &W,
            'x' => &X,
            'y' => &Y,
            'z' => &Z,
            ' ' => &SPACE,
            '.' => &PERIOD,
            ':' => &COLON,
            '-' => &MINUS,
            '+' => &PLUS,
            '/' => &SLASH,
            '%' => &PERCENT,
            '(' => &LEFT_PARENTHESIS,
            ')' => &RIGHT_PARENTHESIS,
            '|' => &VERTICAL_BAR,
            '<' => &LESS_THAN,
            '>' => &GREATER_THAN,
            _ => &MISSING
        }
    }

    pub fn width(&self) -> u32 {
        WIDTH
    }

    pub fn lit_pixels(&self) -> impl Iterator<Item = (u32, u32)> + '_ {
        self.rows.iter().enumerate().flat_map(|(row, pixels)| {
            pixels
                .bytes()
                .enumerate()
                .filter(|(_, pixel)| *pixel == LIT)
                .map(move |(column, _)| (column as u32, row as u32))
        })
    }
}

#[rustfmt::skip]
const DIGIT_0: Glyph = Glyph::new([
    ".###.",
    "#...#",
    "#..##",
    "#.#.#",
    "##..#",
    "#...#",
    ".###.",
    ".....",
]);

#[rustfmt::skip]
const DIGIT_1: Glyph = Glyph::new([
    "..#..",
    ".##..",
    "..#..",
    "..#..",
    "..#..",
    "..#..",
    "#####",
    ".....",
]);

#[rustfmt::skip]
const DIGIT_2: Glyph = Glyph::new([
    ".###.",
    "#...#",
    "....#",
    "..##.",
    ".#...",
    "#...#",
    "#####",
    ".....",
]);

#[rustfmt::skip]
const DIGIT_3: Glyph = Glyph::new([
    ".###.",
    "#...#",
    "....#",
    "..##.",
    "....#",
    "#...#",
    ".###.",
    ".....",
]);

#[rustfmt::skip]
const DIGIT_4: Glyph = Glyph::new([
    "...##",
    "..#.#",
    ".#..#",
    "#...#",
    "#####",
    "....#",
    "....#",
    ".....",
]);

#[rustfmt::skip]
const DIGIT_5: Glyph = Glyph::new([
    "#####",
    "#....",
    "####.",
    "....#",
    "....#",
    "#...#",
    ".###.",
    ".....",
]);

#[rustfmt::skip]
const DIGIT_6: Glyph = Glyph::new([
    "..##.",
    ".#...",
    "#....",
    "####.",
    "#...#",
    "#...#",
    ".###.",
    ".....",
]);

#[rustfmt::skip]
const DIGIT_7: Glyph = Glyph::new([
    "#####",
    "#...#",
    "....#",
    "...#.",
    "..#..",
    "..#..",
    "..#..",
    ".....",
]);

#[rustfmt::skip]
const DIGIT_8: Glyph = Glyph::new([
    ".###.",
    "#...#",
    "#...#",
    ".###.",
    "#...#",
    "#...#",
    ".###.",
    ".....",
]);

#[rustfmt::skip]
const DIGIT_9: Glyph = Glyph::new([
    ".###.",
    "#...#",
    "#...#",
    ".####",
    "....#",
    "...#.",
    ".##..",
    ".....",
]);

#[rustfmt::skip]
const A: Glyph = Glyph::new([
    ".....",
    ".....",
    ".###.",
    "....#",
    ".####",
    "#...#",
    ".####",
    ".....",
]);

#[rustfmt::skip]
const B: Glyph = Glyph::new([
    "#....",
    "#....",
    "#.##.",
    "##..#",
    "#...#",
    "#...#",
    "####.",
    ".....",
]);

#[rustfmt::skip]
const C: Glyph = Glyph::new([
    ".....",
    ".....",
    ".###.",
    "#...#",
    "#....",
    "#...#",
    ".###.",
    ".....",
]);

#[rustfmt::skip]
const D: Glyph = Glyph::new([
    "....#",
    "....#",
    ".##.#",
    "#..##",
    "#...#",
    "#...#",
    ".####",
    ".....",
]);

#[rustfmt::skip]
const E: Glyph = Glyph::new([
    ".....",
    ".....",
    ".###.",
    "#...#",
    "#####",
    "#....",
    ".####",
    ".....",
]);

#[rustfmt::skip]
const F: Glyph = Glyph::new([
    "..##.",
    ".#...",
    "####.",
    ".#...",
    ".#...",
    ".#...",
    ".#...",
    ".....",
]);

#[rustfmt::skip]
const G: Glyph = Glyph::new([
    ".....",
    ".....",
    ".####",
    "#...#",
    "#...#",
    ".####",
    "....#",
    "####.",
]);

#[rustfmt::skip]
const H: Glyph = Glyph::new([
    "#....",
    "#....",
    "#.##.",
    "##..#",
    "#...#",
    "#...#",
    "#...#",
    ".....",
]);

#[rustfmt::skip]
const I: Glyph = Glyph::new([
    "..#..",
    ".....",
    "..#..",
    "..#..",
    "..#..",
    "..#..",
    "..#..",
    ".....",
]);

#[rustfmt::skip]
const J: Glyph = Glyph::new([
    "....#",
    ".....",
    "....#",
    "....#",
    "....#",
    "....#",
    "#...#",
    ".###.",
]);

#[rustfmt::skip]
const K: Glyph = Glyph::new([
    "#....",
    "#....",
    "#..#.",
    "#.#..",
    "##...",
    "#.#..",
    "#..#.",
    ".....",
]);

#[rustfmt::skip]
const L: Glyph = Glyph::new([
    ".#...",
    ".#...",
    ".#...",
    ".#...",
    ".#...",
    ".#...",
    "..#..",
    ".....",
]);

#[rustfmt::skip]
const M: Glyph = Glyph::new([
    ".....",
    ".....",
    "##.#.",
    "#.#.#",
    "#.#.#",
    "#.#.#",
    "#.#.#",
    ".....",
]);

#[rustfmt::skip]
const N: Glyph = Glyph::new([
    ".....",
    ".....",
    "####.",
    "#...#",
    "#...#",
    "#...#",
    "#...#",
    ".....",
]);

#[rustfmt::skip]
const O: Glyph = Glyph::new([
    ".....",
    ".....",
    ".###.",
    "#...#",
    "#...#",
    "#...#",
    ".###.",
    ".....",
]);

#[rustfmt::skip]
const P: Glyph = Glyph::new([
    ".....",
    ".....",
    "#.##.",
    "##..#",
    "#...#",
    "####.",
    "#....",
    "#....",
]);

#[rustfmt::skip]
const Q: Glyph = Glyph::new([
    ".....",
    ".....",
    ".##.#",
    "#..##",
    "#...#",
    ".####",
    "....#",
    "....#",
]);

#[rustfmt::skip]
const R: Glyph = Glyph::new([
    ".....",
    ".....",
    "#.##.",
    "##..#",
    "#....",
    "#....",
    "#....",
    ".....",
]);

#[rustfmt::skip]
const S: Glyph = Glyph::new([
    ".....",
    ".....",
    ".####",
    "#....",
    ".###.",
    "....#",
    "####.",
    ".....",
]);

#[rustfmt::skip]
const T: Glyph = Glyph::new([
    "..#..",
    "..#..",
    ".###.",
    "..#..",
    "..#..",
    "..#..",
    "...#.",
    ".....",
]);

#[rustfmt::skip]
const U: Glyph = Glyph::new([
    ".....",
    ".....",
    "#...#",
    "#...#",
    "#...#",
    "#...#",
    ".####",
    ".....",
]);

#[rustfmt::skip]
const V: Glyph = Glyph::new([
    ".....",
    ".....",
    "#...#",
    "#...#",
    "#...#",
    ".#.#.",
    "..#..",
    ".....",
]);

#[rustfmt::skip]
const W: Glyph = Glyph::new([
    ".....",
    ".....",
    "#...#",
    "#...#",
    "#.#.#",
    "#.#.#",
    ".####",
    ".....",
]);

#[rustfmt::skip]
const X: Glyph = Glyph::new([
    ".....",
    ".....",
    "#...#",
    ".#.#.",
    "..#..",
    ".#.#.",
    "#...#",
    ".....",
]);

#[rustfmt::skip]
const Y: Glyph = Glyph::new([
    ".....",
    ".....",
    "#...#",
    "#...#",
    "#...#",
    ".####",
    "....#",
    "####.",
]);

#[rustfmt::skip]
const Z: Glyph = Glyph::new([
    ".....",
    ".....",
    "#####",
    "...#.",
    "..#..",
    ".#...",
    "#####",
    ".....",
]);

#[rustfmt::skip]
const SPACE: Glyph = Glyph::new([
    ".....",
    ".....",
    ".....",
    ".....",
    ".....",
    ".....",
    ".....",
    ".....",
]);

#[rustfmt::skip]
const PERIOD: Glyph = Glyph::new([
    ".....",
    ".....",
    ".....",
    ".....",
    ".....",
    ".....",
    "..#..",
    ".....",
]);

#[rustfmt::skip]
const COLON: Glyph = Glyph::new([
    ".....",
    ".....",
    "..#..",
    ".....",
    ".....",
    ".....",
    "..#..",
    ".....",
]);

#[rustfmt::skip]
const MINUS: Glyph = Glyph::new([
    ".....",
    ".....",
    ".....",
    "#####",
    ".....",
    ".....",
    ".....",
    ".....",
]);

#[rustfmt::skip]
const PLUS: Glyph = Glyph::new([
    ".....",
    "..#..",
    "..#..",
    "#####",
    "..#..",
    "..#..",
    ".....",
    ".....",
]);

#[rustfmt::skip]
const SLASH: Glyph = Glyph::new([
    "....#",
    "...#.",
    "...#.",
    "..#..",
    ".#...",
    ".#...",
    "#....",
    ".....",
]);

#[rustfmt::skip]
const PERCENT: Glyph = Glyph::new([
    "#...#",
    "....#",
    "...#.",
    "..#..",
    ".#...",
    "#....",
    "#...#",
    ".....",
]);

#[rustfmt::skip]
const LEFT_PARENTHESIS: Glyph = Glyph::new([
    "...##",
    "..#..",
    ".#...",
    ".#...",
    ".#...",
    "..#..",
    "...##",
    ".....",
]);

#[rustfmt::skip]
const RIGHT_PARENTHESIS: Glyph = Glyph::new([
    "##...",
    "..#..",
    "...#.",
    "...#.",
    "...#.",
    "..#..",
    "##...",
    ".....",
]);

#[rustfmt::skip]
const VERTICAL_BAR: Glyph = Glyph::new([
    "..#..",
    "..#..",
    "..#..",
    "..#..",
    "..#..",
    "..#..",
    "..#..",
    "..#..",
]);

#[rustfmt::skip]
const LESS_THAN: Glyph = Glyph::new([
    ".....",
    "...##",
    ".##..",
    "#....",
    ".##..",
    "...##",
    ".....",
    ".....",
]);

#[rustfmt::skip]
const GREATER_THAN: Glyph = Glyph::new([
    ".....",
    "##...",
    "..##.",
    "....#",
    "..##.",
    "##...",
    ".....",
    ".....",
]);

#[rustfmt::skip]
const MISSING: Glyph = Glyph::new([
    "#####",
    "#...#",
    "#...#",
    "#...#",
    "#...#",
    "#...#",
    "#####",
    ".....",
]);

#[cfg(test)]
mod tests {
    use super::*;

    const CHARACTERS: &str = "abcdefghijklmnopqrstuvwxyz0123456789 .:-+/%()|<>";

    #[test]
    fn every_glyph_fills_the_monospace_cell() {
        for character in CHARACTERS.chars().chain(['?']) {
            let glyph = Glyph::of(character);

            assert!(
                glyph.rows.iter().all(|row| row.len() == WIDTH as usize),
                "{character:?} is not {WIDTH} pixels wide"
            );
        }
    }
}
