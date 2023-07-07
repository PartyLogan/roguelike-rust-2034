use macroquad::prelude::*;

pub fn get_xy(x: i32, y: i32, width: i32) -> usize {
    ((y * width) + x) as usize
}

pub fn get_x_and_y(index: usize, width: i32) -> (i32, i32) {
    let x = index % width as usize;
    let y = index / width as usize;
    (x as i32, y as i32)
}

pub fn get_glyph_coords(glyph: char, cell_size: f32) -> Vec2 {
    let mut result: Vec2 = Vec2::new(0.0, 0.0);
    match glyph {
        '☻' => result.x = 2.0, // ☺ - Player
        '#' => {
            // '#' - Wall
            result.x = 3.0;
            result.y = 2.0
        }
        '@' => result.y = 4.0, // @
        '.' => {
            // . Floor
            result.y = 2.0;
            result.x = 14.0;
        }
        _ => result.y = 2.0, // Blank
    }
    result.x *= cell_size;
    result.y *= cell_size;
    return result;
}
