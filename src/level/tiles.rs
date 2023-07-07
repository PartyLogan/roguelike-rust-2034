use macroquad::prelude::*;

use crate::console::cell::Cell;

#[derive(Clone, Debug, PartialEq)]
pub struct Tile {
    pub name: String,
    pub cell: Cell,
    pub passable: bool,
    pub transparent: bool,
}

impl Tile {
    pub fn new(name: String, cell: Cell, passable: bool, transparent: bool) -> Self {
        Tile {
            name,
            cell,
            passable,
            transparent,
        }
    }

    pub fn make_wall() -> Self {
        Tile::new(
            "Wall".to_string(),
            Cell::new('#', BLACK, WHITE),
            false,
            false,
        )
    }

    pub fn make_floor() -> Self {
        Tile::new(
            "Floor".to_string(),
            Cell::new('.', BLACK, WHITE),
            true,
            true,
        )
    }
}
