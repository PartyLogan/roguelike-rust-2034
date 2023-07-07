use crate::console::cell::Cell;
use ::rand::{rngs::ThreadRng, Rng};
use macroquad::prelude::*;

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
            Cell::new('▲', DARKGREEN, GREEN),
            false,
            false,
        )
    }

    pub fn make_floor(rng: &mut ThreadRng) -> Self {
        let v = rng.gen_range(0..=4);

        let mut char = '.';
        match v {
            0 => char = '.',
            1 => char = ',',
            2 => char = '`',
            3 => char = '"',
            _ => char = '^',
        }
        Tile::new(
            "Floor".to_string(),
            Cell::new(char, BROWN, LIME),
            true,
            true,
        )
    }
}
