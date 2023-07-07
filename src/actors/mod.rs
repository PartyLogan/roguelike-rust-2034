use macroquad::prelude::*;

use crate::{
    actions::{movement::BumpAction, Action},
    console::cell::Cell,
    util::get_glyph_coords,
};

pub const DELAY_TIME: usize = 10;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActorType {
    Player,
    Enemy,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Actor {
    pub x: i32,
    pub y: i32,
    pub cell: Cell,
    pub render: bool,
    pub actor_type: ActorType,
}

impl Actor {
    pub fn render(&self, texture: &Texture2D, cell_size: i32) {
        if self.render != true {
            return;
        }

        let x = self.x * cell_size;
        let y = self.y * cell_size;
        draw_rectangle(
            x as f32,
            y as f32,
            cell_size as f32,
            cell_size as f32,
            self.cell.bg,
        );

        let texture_pos = get_glyph_coords(self.cell.glyph, cell_size as f32);

        draw_texture_ex(
            *texture,
            x as f32,
            y as f32,
            self.cell.fg,
            DrawTextureParams {
                dest_size: Some(vec2(cell_size as f32, cell_size as f32)),
                source: Some(Rect::new(
                    texture_pos.x,
                    texture_pos.y,
                    cell_size as f32,
                    cell_size as f32,
                )),
                ..Default::default()
            },
        );
    }

    pub fn get_action(&mut self) -> Option<Box<dyn Action>> {
        if self.actor_type == ActorType::Player {
            let action = self.get_player_input();
            return self.get_player_input();
        }
        None
    }

    pub fn get_player_input(&self) -> Option<Box<dyn Action>> {
        if is_key_down(KeyCode::Up) {
            return Some(Box::new(BumpAction::new(0, -1)));
        }
        if is_key_down(KeyCode::Down) {
            return Some(Box::new(BumpAction::new(0, 1)));
        }
        if is_key_down(KeyCode::Left) {
            return Some(Box::new(BumpAction::new(-1, 0)));
        }
        if is_key_down(KeyCode::Right) {
            return Some(Box::new(BumpAction::new(1, 0)));
        }

        None
    }
}
