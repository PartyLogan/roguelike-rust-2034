use raylib::prelude::Color;

#[derive(Clone, Copy)]
pub struct Cell {
    pub glyph: char,
    pub bg: Color,
    pub fg: Color,
}
