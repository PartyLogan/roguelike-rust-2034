use macroquad::prelude::*;

use crate::{
    fov::FOV,
    level::Level,
    util::{get_glyph_coords, get_xy},
};

pub mod cell;

pub struct Console {
    pub width: i32,
    pub height: i32,
    pub cell_size: i32,
    pub render: bool,
}

impl Console {
    pub fn render(&self, texture: &Texture2D, level: &Level, fov: &FOV) {
        if self.render != true {
            return;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = level.get_cell(get_xy(x, y, self.width));

                if fov.visible[get_xy(x, y, self.width)] == false
                    && fov.seen[get_xy(x, y, self.width)] == false
                {
                    continue;
                }

                let dx = x * self.cell_size;
                let dy = y * self.cell_size;

                // draw_rectangle(
                //     dx as f32,
                //     dy as f32,
                //     self.cell_size as f32,
                //     self.cell_size as f32,
                //     cell.bg,
                // );

                let texture_pos = get_glyph_coords(cell.glyph, self.cell_size as f32);
                let blank_pos = get_glyph_coords(' ', self.cell_size as f32);
                if fov.visible[get_xy(x, y, self.width)] {
                    draw_texture_ex(
                        *texture,
                        dx as f32,
                        dy as f32,
                        cell.bg,
                        DrawTextureParams {
                            dest_size: Some(vec2(self.cell_size as f32, self.cell_size as f32)),
                            source: Some(Rect::new(
                                blank_pos.x,
                                blank_pos.y,
                                self.cell_size as f32,
                                self.cell_size as f32,
                            )),
                            ..Default::default()
                        },
                    );
                    draw_texture_ex(
                        *texture,
                        dx as f32,
                        dy as f32,
                        cell.fg,
                        DrawTextureParams {
                            dest_size: Some(vec2(self.cell_size as f32, self.cell_size as f32)),
                            source: Some(Rect::new(
                                texture_pos.x,
                                texture_pos.y,
                                self.cell_size as f32,
                                self.cell_size as f32,
                            )),
                            ..Default::default()
                        },
                    );
                } else {
                    draw_texture_ex(
                        *texture,
                        dx as f32,
                        dy as f32,
                        DARKGRAY,
                        DrawTextureParams {
                            dest_size: Some(vec2(self.cell_size as f32, self.cell_size as f32)),
                            source: Some(Rect::new(
                                blank_pos.x,
                                blank_pos.y,
                                self.cell_size as f32,
                                self.cell_size as f32,
                            )),
                            ..Default::default()
                        },
                    );
                    draw_texture_ex(
                        *texture,
                        dx as f32,
                        dy as f32,
                        LIGHTGRAY,
                        DrawTextureParams {
                            dest_size: Some(vec2(self.cell_size as f32, self.cell_size as f32)),
                            source: Some(Rect::new(
                                texture_pos.x,
                                texture_pos.y,
                                self.cell_size as f32,
                                self.cell_size as f32,
                            )),
                            ..Default::default()
                        },
                    );
                }
            }
        }
    }
}
