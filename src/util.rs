use macroquad::prelude::*;

use crate::level::tiles::TileType;

pub fn get_xy(x: i32, y: i32, width: i32) -> usize {
    ((y * width) + x) as usize
}

pub fn get_glyph_coords(glyph: char, cell_size: f32) -> Vec2 {
    let mut result: Vec2 = Vec2::new(0.0, 0.0);
    match glyph {
        '☻' => result.x = 25.0, // ☺ - Player
        '▲' => {
            // '#' - Tree
            result.x = 3.0;
            result.y = 2.0
        }
        '@' => result.y = 4.0, // @
        '.' => {
            // . Floor
            result.y = 0.0;
            result.x = 5.0;
        }
        ',' => {
            // Floor 2
            result.x = 1.0;
        }
        '`' => {
            // Floor 3
            result.x = 7.0;
        }
        '"' => {
            // Floor 4
            result.x = 6.0;
        }
        '^' => {
            // Floor 4
            result.y = 2.0;
        }
        _ => {
            // Blank
            result.x = 0.0;
            result.y = 0.0;
        }
    }
    result.x *= cell_size;
    result.y *= cell_size;
    return result;
}

pub fn get_tile_glyph_coords(tile_type: TileType, cell_size: f32) -> Vec2 {
    let mut result: Vec2 = Vec2::new(0.0, 0.0);
    match tile_type {
        TileType::Wall1 => {
            result.x = 3.0;
            result.y = 2.0
        }
        TileType::Wall2 => {
            result.x = 3.0;
            result.y = 1.0
        }
        TileType::Wall3 => {
            result.x = 2.0;
            result.y = 1.0
        }
        TileType::Wall4 => {
            result.x = 4.0;
            result.y = 2.0
        }
        TileType::Wall5 => {
            result.x = 1.0;
            result.y = 1.0
        }
        TileType::Floor1 => {
            result.y = 0.0;
            result.x = 5.0;
        }
        TileType::Floor2 => {
            result.x = 1.0;
        }
        TileType::Floor3 => {
            result.x = 7.0;
        }
        TileType::Floor4 => {
            result.x = 6.0;
        }
        TileType::Floor5 => {
            result.y = 2.0;
        }
        TileType::FloorBlank => {
            // Blank
            result.x = 0.0;
            result.y = 0.0;
        }
        _ => {
            // Blank
            result.x = 8.0;
            result.y = 5.0;
        }
    }
    result.x *= cell_size;
    result.y *= cell_size;
    return result;
}
