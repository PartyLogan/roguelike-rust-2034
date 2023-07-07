use crate::{level::Level, util::get_x_and_y};

#[derive(Clone, Debug, PartialEq)]
pub struct FOV {
    pub view_range: i32,
    pub seen: Vec<bool>,
    pub visible: Vec<bool>,
}

impl FOV {
    pub fn new(range: i32, width: i32, height: i32) -> FOV {
        FOV {
            view_range: range,
            seen: vec![false; (width * height) as usize],
            visible: vec![false; (width * height) as usize],
        }
    }

    pub fn update(&mut self, x: i32, y: i32, level: &Level) {
        for i in 0..level.tiles.len() {
            let (tile_x, tile_y) = get_x_and_y(i, level.width);
            if tile_x > x - self.view_range
                && tile_x < x + self.view_range
                && tile_y > y - self.view_range
                && tile_y < y + self.view_range
            {
                self.seen[i] = true;
                self.visible[i] = true;
            } else {
                self.visible[i] = false;
            }
        }
    }
}

pub fn update_fov(fov: &mut FOV, x: i32, y: i32, level: &Level) {
    fov.update(x, y, level);
}
