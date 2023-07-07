use raylib::{
    prelude::{KeyboardKey, RaylibDraw, RaylibDrawHandle, Rectangle, Vector2},
    texture::Texture2D,
    RaylibHandle,
};

use crate::{
    actions::{movement::BumpAction, Action},
    console::cell::Cell,
    util::get_glyph_coords,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActorType {
    Player,
    Enemy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Actor {
    pub x: i32,
    pub y: i32,
    pub cell: Cell,
    pub render: bool,
    pub actor_type: ActorType,
}

impl Actor {
    pub fn render(&self, d: &mut RaylibDrawHandle, texture: &Texture2D, cell_size: i32) {
        if self.render != true {
            return;
        }

        let x = self.x * cell_size;
        let y = self.y * cell_size;
        d.draw_rectangle(x, y, cell_size, cell_size, self.cell.bg);

        let texture_pos = get_glyph_coords(self.cell.glyph, cell_size as f32);

        d.draw_texture_rec(
            texture,
            Rectangle::new(
                texture_pos.x,
                texture_pos.y,
                cell_size as f32,
                cell_size as f32,
            ),
            Vector2::new(x as f32, y as f32),
            self.cell.fg,
        );
    }

    pub fn get_action(&mut self, rl: &mut RaylibHandle) -> Option<Box<dyn Action>> {
        if self.actor_type == ActorType::Player {
            return self.get_player_input(rl);
        }
        None
    }

    pub fn get_player_input(&self, rl: &mut RaylibHandle) -> Option<Box<dyn Action>> {
        if rl.is_key_pressed(KeyboardKey::KEY_UP) {
            return Some(Box::new(BumpAction::new(0, -1)));
        }
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
            return Some(Box::new(BumpAction::new(0, 1)));
        }
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
            return Some(Box::new(BumpAction::new(-1, 0)));
        }
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            return Some(Box::new(BumpAction::new(1, 0)));
        }

        None
    }
}
