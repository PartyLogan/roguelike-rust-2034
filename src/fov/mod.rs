use crate::{level::Level, util::get_xy};

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
        for u in self.visible.iter_mut() {
            *u = false;
        }
        for i in -self.view_range..=self.view_range {
            for j in -self.view_range..=self.view_range {
                if i * i + j * j < self.view_range * self.view_range {
                    los(self, x, y, x + i, y + j, level);
                }
            }
        }
    }
}

pub fn update_fov(fov: &mut FOV, x: i32, y: i32, level: &Level) {
    fov.update(x, y, level);
    fov.seen[get_xy(x, y, level.width) as usize] = true;
    fov.visible[get_xy(x, y, level.width) as usize] = true;
}

pub fn los(fov: &mut FOV, x0: i32, y0: i32, x1: i32, y1: i32, level: &Level) {
    let mut _sx = 0;
    let mut _sy = 0;
    let mut _xnext = 0;
    let mut _ynext = 0;
    let mut _dx = 0;
    let mut _dy = 0;
    let mut _dist: f32 = 0.;

    _dx = x1 - x0;
    _dy = y1 - y0;

    //determine which quadrant to we're calculating: we climb in these two directions
    _sx = if x0 < x1 { 1 } else { -1 };
    _sy = if y0 < y1 { 1 } else { -1 };

    _xnext = x0;
    _ynext = y0;

    // calculate length of the line to cast
    _dist = f32::sqrt(_dx as f32 * _dx as f32 + _dy as f32 * _dy as f32) as f32;

    while _xnext != x1 || _ynext != y1 {
        // casting a ray of length radius
        if level.in_bounds(_xnext, _ynext) {
            let tile = level.get_tile(get_xy(_xnext, _ynext, level.width));
            if tile.transparent == false {
                fov.visible[get_xy(_xnext, _ynext, level.width) as usize] = true;
                fov.seen[get_xy(_xnext, _ynext, level.width) as usize] = true;
                return;
            }

            // Line-to-point distance formula < 0.5
            if i32::abs(_dy * (_xnext - x0 + _sx) - _dx * (_ynext - y0)) as f32 / _dist < 0.5 {
                _xnext += _sx;
            } else if i32::abs(_dy * (_xnext - x0) - _dx * (_ynext - y0 + _sy)) as f32 / _dist < 0.5
            {
                _ynext += _sy;
            } else {
                _xnext += _sx;
                _ynext += _sy;
            }
        }
        fov.visible[get_xy(_xnext, _ynext, level.width) as usize] = true;
        fov.seen[get_xy(_xnext, _ynext, level.width) as usize] = true;
    }
}
