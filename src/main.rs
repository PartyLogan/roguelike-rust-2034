use actors::{Actor, ActorType};
use console::{cell::Cell, Console};
use level::Level;
use raylib::prelude::*;

mod actions;
mod actors;
mod console;
mod level;
mod util;

pub const SCREEN_WIDTH: i32 = 1280;
pub const SCREEN_HEIGHT: i32 = 720;
pub const TILE_SIZE: i32 = 16;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Roguelike")
        .build();

    let texture = rl
        .load_texture(&thread, "assets/Anikki_square_16x16.png")
        .expect("Could not load texture");

    let mut gamestate = GameState::new(texture);
    let mut player = Actor {
        x: 0,
        y: 0,
        cell: Cell {
            glyph: '☻',
            bg: Color::BLACK,
            fg: Color::YELLOW,
        },
        render: true,
        actor_type: ActorType::Player,
    };

    let (px, py) = gamestate.level.generate_basic_dungeon();

    player.x = px;
    player.y = py;

    gamestate.actors.push(player);

    while !rl.window_should_close() {
        update(&mut gamestate, &mut rl);
        render(&mut gamestate, &mut rl, &thread);
    }
}

pub fn update(gamestate: &mut GameState, rl: &mut RaylibHandle) {
    gamestate.update(rl);
}

pub fn render(gamestate: &mut GameState, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(&thread);
    gamestate.render(&mut d);
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
        };
        return s;
    }

    pub fn render(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);

        self.root_console.render(d, &self.tilemap, &self.level);

        for actor in self.actors.iter() {
            actor.render(d, &self.tilemap, self.tilesize);
        }

        d.draw_fps(20, 20);
    }

    pub fn update(&mut self, rl: &mut RaylibHandle) {
        let mut action = self.actors[self.current_actor].get_action(rl);
        if action.is_none() {
            return;
        }

        loop {
            let result = action
                .as_mut()
                .unwrap()
                .execute(&mut self.actors[self.current_actor], &mut self.level);

            if result.success {
                return;
            }
            if result.alternative.is_none() {
                break;
            }
            action = result.alternative;
        }

        self.current_actor = (self.current_actor + 1) % self.actors.len();
    }
}
