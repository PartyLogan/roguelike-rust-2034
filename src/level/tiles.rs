use crate::console::cell::Cell;
use ::rand::{rngs::ThreadRng, Rng};
use macroquad::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    pub name: String,
    pub cell: Cell,
    pub tile_type: TileType,
    pub passable: bool,
    pub transparent: bool,
}

impl Tile {
    pub fn new(
        name: String,
        cell: Cell,
        tile_type: TileType,
        passable: bool,
        transparent: bool,
    ) -> Self {
        Tile {
            name,
            cell,
            tile_type,
            passable,
            transparent,
        }
    }

    pub fn make_wall(rng: &mut ThreadRng) -> Self {
        let v = rng.gen_range(0..=4);

        let mut _t: TileType = TileType::WallBlank;
        match v {
            0 => _t = TileType::Wall1,
            1 => _t = TileType::Wall2,
            2 => _t = TileType::Wall3,
            3 => _t = TileType::Wall4,
            _ => _t = TileType::Wall3,
        }

        Tile::new(
            "Wall".to_string(),
            Cell::new('#', DARKGREEN, GREEN),
            _t,
            false,
            false,
        )
    }

    pub fn make_floor(rng: &mut ThreadRng) -> Self {
        let v = rng.gen_range(0..=5);

        let mut _t = TileType::FloorBlank;
        match v {
            0 => _t = TileType::Floor1,
            1 => _t = TileType::Floor2,
            2 => _t = TileType::Floor3,
            3 => _t = TileType::Floor4,
            4 => _t = TileType::Floor5,
            _ => _t = TileType::FloorBlank,
        }
        Tile::new(
            "Floor".to_string(),
            Cell::new('.', DARKBROWN, LIME),
            _t,
            true,
            true,
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileType {
    Blank,
    FloorBlank,
    Floor1,
    Floor2,
    Floor3,
    Floor4,
    Floor5,
    WallBlank,
    Wall1,
    Wall2,
    Wall3,
    Wall4,
    Wall5,
}
