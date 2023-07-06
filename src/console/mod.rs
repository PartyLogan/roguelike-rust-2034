use raylib::{
    prelude::{RaylibDraw, RaylibDrawHandle, Rectangle, Vector2},
    texture::Texture2D,
};

use crate::util::{get_glyph_coords, get_xy};

use self::cell::Cell;

pub mod cell;

pub struct Console {
    pub width: i32,
    pub height: i32,
    pub cell_size: i32,
    pub cells: Vec<Cell>,
    pub render: bool,
}

impl Console {
    pub fn set_cell(&mut self, x: i32, y: i32, cell: Cell) {
        self.cells[get_xy(x, y, self.width) as usize] = cell;
    }

    pub fn render(&self, d: &mut RaylibDrawHandle, texture: &Texture2D) {
        if self.render != true {
            return;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.cells[get_xy(x, y, self.width) as usize];

                let dx = x * self.cell_size;
                let dy = y * self.cell_size;
                d.draw_rectangle(dx, dy, self.cell_size, self.cell_size, cell.bg);

                let texture_pos = get_glyph_coords(cell.glyph, self.cell_size as f32);

                d.draw_texture_rec(
                    texture,
                    Rectangle::new(
                        texture_pos.x,
                        texture_pos.y,
                        self.cell_size as f32,
                        self.cell_size as f32,
                    ),
                    Vector2::new(dx as f32, dy as f32),
                    cell.fg,
                );
            }
        }
    }
}
