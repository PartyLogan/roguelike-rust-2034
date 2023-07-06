use raylib::{
    prelude::{Color, RaylibDraw, RaylibDrawHandle, Rectangle, Vector2},
    texture::Texture2D,
};

use crate::util::get_glyph_coords;

pub struct Actor {
    pub x: i32,
    pub y: i32,
    pub glyph: char,
    pub bg: Color,
    pub fg: Color,
    pub render: bool,
}

impl Actor {
    pub fn render(&self, d: &mut RaylibDrawHandle, texture: &Texture2D, cell_size: i32) {
        if self.render != true {
            return;
        }

        let x = self.x * cell_size;
        let y = self.y * cell_size;
        d.draw_rectangle(x, y, cell_size, cell_size, self.bg);

        let texture_pos = get_glyph_coords(self.glyph, cell_size as f32);

        d.draw_texture_rec(
            texture,
            Rectangle::new(
                texture_pos.x,
                texture_pos.y,
                cell_size as f32,
                cell_size as f32,
            ),
            Vector2::new(x as f32, y as f32),
            self.fg,
        );
    }
}
