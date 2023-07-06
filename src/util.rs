use raylib::prelude::Vector2;

pub fn get_xy(x: i32, y: i32, width: i32) -> i32 {
    (y * width) + x
}

pub fn get_glyph_coords(glyph: char, cell_size: f32) -> Vector2 {
    let value = glyph as u32;
    match value {
        23 => Vector2::new(4.0 * cell_size, 3.0 * cell_size), // '#'
        64 => Vector2::new(0.0, 4.0 * cell_size),             // @

        _ => Vector2::new(0.0, 2.0 * cell_size), // Blank
    }
}
