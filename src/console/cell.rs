use raylib::prelude::Color;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Cell {
    pub glyph: char,
    pub bg: Color,
    pub fg: Color,
}

impl Cell {
    pub fn new(glyph: char, bg: Color, fg: Color) -> Self {
        Cell { glyph, bg, fg }
    }
}
