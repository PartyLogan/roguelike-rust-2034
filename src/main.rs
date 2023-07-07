use fov::{update_fov, FOV};
use macroquad::prelude::*;

use actors::{Actor, ActorType};
use console::{cell::Cell, Console};
use level::{tiles::Tile, Level};
use util::get_xy;

mod actions;
mod actors;
mod console;
mod fov;
mod level;
mod util;

pub const SCREEN_WIDTH: i32 = 1280;
pub const SCREEN_HEIGHT: i32 = 720;
pub const TILE_SIZE: i32 = 16;

#[macroquad::main("Roguelike")]
async fn main() {
    request_new_screen_size(SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32);

    let texture = load_texture("assets/monochrome-transparent_packed.png")
        .await
        .unwrap();
    texture.set_filter(FilterMode::Nearest);

    let mut gamestate = GameState::new(texture);
    gamestate.level.generate_basic_dungeon();

    let player = Actor {
        x: gamestate.level.start_x,
        y: gamestate.level.start_y,
        cell: Cell {
            glyph: '☻',
            bg: BLACK,
            fg: YELLOW,
        },
        render: true,
        actor_type: ActorType::Player,
        fov: fov::FOV::new(6, SCREEN_WIDTH / TILE_SIZE, SCREEN_HEIGHT / TILE_SIZE),
    };

    gamestate.actors.push(player);

    update_fov(
        &mut gamestate.render_fov,
        gamestate.level.start_x,
        gamestate.level.start_y,
        &gamestate.level,
    );

    loop {
        gamestate.update();
        gamestate.render();

        next_frame().await
    }
}

pub struct GameState {
    pub tilemap: Texture2D,
    pub tilesize: i32,
    pub width: i32,
    pub height: i32,
    pub root_console: Console,
    pub level: Level,
    pub actors: Vec<Actor>,
    pub current_actor: usize,
    pub render_fov: FOV,
}

impl GameState {
    pub fn new(texture: Texture2D) -> GameState {
        let s = GameState {
            tilemap: texture,
            tilesize: TILE_SIZE,
            width: SCREEN_WIDTH / TILE_SIZE,
            height: SCREEN_HEIGHT / TILE_SIZE,
            root_console: Console {
                width: SCREEN_WIDTH / TILE_SIZE,
                height: SCREEN_HEIGHT / TILE_SIZE,
                cell_size: TILE_SIZE,
                render: true,
            },
            level: Level::new(SCREEN_WIDTH / TILE_SIZE, SCREEN_HEIGHT / TILE_SIZE),
            actors: Vec::new(),
            current_actor: 0,
            render_fov: fov::FOV::new(12, SCREEN_WIDTH / TILE_SIZE, SCREEN_HEIGHT / TILE_SIZE),
        };

        return s;
    }

    pub fn render(&self) {
        clear_background(BLACK);

        self.root_console
            .render(&self.tilemap, &self.level, &self.render_fov);

        for actor in self.actors.iter() {
            actor.render(&self.tilemap, self.tilesize);
        }

        draw_rectangle(0.0, 0.0, 50.0, 50.0, BLACK);
        draw_text(get_fps().to_string().as_str(), 0.0, 30.0, 40.0, GREEN);
    }

    pub fn update(&mut self) {
        let x = self.actors[self.current_actor].x;
        let y = self.actors[self.current_actor].y;

        let mut action = self.actors[self.current_actor].get_action();
        if action.is_none() {
            return;
        }
        let mut new_actor: bool = false;
        loop {
            let result = action
                .as_mut()
                .unwrap()
                .execute(&mut self.actors[self.current_actor], &mut self.level);

            if result.success {
                new_actor = true;
                break;
            }
            if result.alternative.is_none() {
                break;
            }
            action = result.alternative;
        }

        self.actors[self.current_actor]
            .fov
            .update(x, y, &self.level);

        if self.actors[self.current_actor].actor_type == ActorType::Player {
            update_fov(
                &mut self.render_fov,
                self.actors[self.current_actor].x,
                self.actors[self.current_actor].y,
                &self.level,
            );
        }
        if new_actor {
            self.current_actor = (self.current_actor + 1) % self.actors.len();
        }
    }
}
