pub mod tiles;

use std::cmp;

use rand::rngs::ThreadRng;
pub use rand::Rng;

use crate::{console::cell::Cell, util::get_xy};

use self::tiles::Tile;

pub struct Level {
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Tile>,
    pub start_x: i32,
    pub start_y: i32,
}

impl Level {
    pub fn new(width: i32, height: i32) -> Self {
        Level {
            width,
            height,
            tiles: vec![Tile::make_wall(); (width * height) as usize],
            start_x: 0,
            start_y: 0,
        }
    }

    pub fn get_cell(&self, index: usize) -> Cell {
        return self.tiles[index].cell;
    }

    pub fn get_tile(&self, index: usize) -> &Tile {
        return &self.tiles[index];
    }

    pub fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {
        self.tiles[get_xy(x, y, self.width) as usize] = tile;
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }

    pub fn floor_room(&mut self, room: RectangularRoom, rng: &mut ThreadRng) {
        let (rx1, ry1, rx2, ry2) = room.inner();
        let minx1 = cmp::min(rx1, rx2);
        let maxx1 = cmp::max(rx1, rx2);

        let miny1 = cmp::min(ry1, ry2);
        let maxy1 = cmp::max(ry1, ry2);

        for x in minx1..=maxx1 {
            for y in miny1..=maxy1 {
                self.set_tile(x, y, Tile::make_floor(rng));
            }
        }
    }

    pub fn tunnel_between(&mut self, start: (i32, i32), end: (i32, i32), rng: &mut ThreadRng) {
        let (x1, y1) = start;
        let (x2, y2) = end;

        if x2 > x1 {
            for x in x1..=x2 {
                self.set_tile(x, y1, Tile::make_floor(rng));
            }
        } else {
            for x in x2..=x1 {
                self.set_tile(x, y1, Tile::make_floor(rng));
            }
        }

        if y2 > y1 {
            for y in y1..=y2 {
                self.set_tile(x2, y, Tile::make_floor(rng));
            }
        } else {
            for y in y2..=y1 {
                self.set_tile(x2, y, Tile::make_floor(rng));
            }
        }
    }

    pub fn generate_basic_dungeon(&mut self) {
        let room_max_size = 10;
        let room_min_size = 6;
        let max_rooms = 30;

        let mut player_x: i32 = 0;
        let mut player_y: i32 = 0;

        let mut rooms: Vec<RectangularRoom> = Vec::new();

        let mut rng = rand::thread_rng();

        for _r in 0..max_rooms {
            let room_width = rng.gen_range(room_min_size..=room_max_size);
            let room_height = rng.gen_range(room_min_size..=room_max_size);

            let x = rng.gen_range(1..self.width - room_width - 1);
            let y = rng.gen_range(1..self.height - room_height - 1);

            let new_room = RectangularRoom::new(x, y, room_width, room_height);

            let mut intersects: bool = false;
            for room in &rooms {
                if new_room.intersects(*room) {
                    intersects = true;
                    continue;
                }
            }

            if intersects {
                continue;
            }

            self.floor_room(new_room, &mut rng);

            if rooms.is_empty() {
                // Player co-ords here
                player_x = new_room.center().0;
                player_y = new_room.center().1;
            } else {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = rooms[rooms.len() - 1].center();

                if rng.gen_range(0..2) == 1 {
                    self.tunnel_between((prev_x, new_y), (new_x, new_y), &mut rng);
                    self.tunnel_between((prev_x, prev_y), (prev_x, new_y), &mut rng);
                } else {
                    self.tunnel_between((prev_x, prev_y), (new_x, prev_y), &mut rng);
                    self.tunnel_between((new_x, prev_y), (new_x, new_y), &mut rng);
                }
            }

            rooms.push(new_room);
        }
        self.start_x = player_x;
        self.start_y = player_y;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RectangularRoom {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}
impl RectangularRoom {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        RectangularRoom {
            x1: x,
            y1: y,
            x2: x + w,
            y2: y + h,
        }
    }

    pub fn center(&self) -> (i32, i32) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;

        (center_x, center_y)
    }

    pub fn inner(&self) -> (i32, i32, i32, i32) {
        let inner_x1 = self.x1 + 1;
        let inner_y1 = self.y1 + 1;
        let inner_x2 = self.x2 - 1;
        let inner_y2 = self.y2 - 1;

        (inner_x1, inner_y1, inner_x2, inner_y2)
    }

    pub fn outter(&self) -> RectangularRoom {
        let outter_x1 = self.x1 - 1;
        let outter_y1 = self.y1 - 1;
        let outter_x2 = self.x2 + 1;
        let outter_y2 = self.y2 + 1;

        RectangularRoom::new(outter_x1, outter_y1, outter_x2, outter_y2)
    }

    pub fn intersects(&self, other: RectangularRoom) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }
}
