use raylib::{
    prelude::{Color, KeyboardKey, RaylibDraw, RaylibDrawHandle, Rectangle, Vector2},
    texture::Texture2D,
    RaylibHandle,
};

use crate::{
    actions::{movement::WalkAction, Action},
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
    pub glyph: char,
    pub bg: Color,
    pub fg: Color,
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
        d.draw_rectangle(x, y, cell_size, cell_size, self.bg);

        let texture_pos = get_glyph_coords(self.glyph, cell_size as f32);

        d.draw_texture_rec(
            texture,
            Rectangle::new(
                texture_pos.x,
                texture_pos.y,
                cell_size as f32,
                cell_size as f32,
            ),
            Vector2::new(x as f32, y as f32),
            self.fg,
        );
    }

    pub fn get_action(&mut self, pressed_key: Option<KeyboardKey>) -> Option<Box<dyn Action>> {
        if self.actor_type == ActorType::Player {
            return self.get_player_input(pressed_key);
        }
        None
    }

    pub fn get_player_input(&self, pressed_key: Option<KeyboardKey>) -> Option<Box<dyn Action>> {
        if let Some(pressed_key) = pressed_key {
            match pressed_key {
                KeyboardKey::KEY_UP => return Some(Box::new(WalkAction::new(0, -1))),
                KeyboardKey::KEY_DOWN => return Some(Box::new(WalkAction::new(0, 1))),
                KeyboardKey::KEY_LEFT => return Some(Box::new(WalkAction::new(-1, 0))),
                KeyboardKey::KEY_RIGHT => return Some(Box::new(WalkAction::new(1, 0))),
                _ => return None,
            };
        }
        None
    }
}
