use actors::{Actor, ActorType};
use console::{cell::Cell, Console};
use raylib::prelude::*;

mod actions;
mod actors;
mod console;
mod util;

fn main() {
    let (mut rl, thread) = raylib::init().size(1280, 720).title("Roguelike").build();

    let texture = rl
        .load_texture(&thread, "assets/Anikki_square_16x16.png")
        .expect("Could not load texture");

    let mut gamestate = GameState::new(texture);
    let test_actor = Actor {
        x: 10,
        y: 10,
        glyph: '@',
        bg: Color::BLACK,
        fg: Color::YELLOW,
        render: true,
        actor_type: ActorType::Player,
    };

    gamestate.actors.push(test_actor);

    while !rl.window_should_close() {
        update(&mut gamestate, &mut rl);
        render(&mut gamestate, &mut rl, &thread);
    }
}

pub fn update(gamestate: &mut GameState, rl: &mut RaylibHandle) {
    let pressed_key = rl.get_key_pressed();
    gamestate.update(pressed_key);
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
    pub actors: Vec<Actor>,
    pub current_actor: usize,
}

impl GameState {
    pub fn new(texture: Texture2D) -> GameState {
        let s = GameState {
            tilemap: texture,
            tilesize: 16,
            width: 1280,
            height: 720,
            root_console: Console {
                width: 80,
                height: 50,
                cell_size: 16,
                cells: vec![
                    Cell {
                        glyph: 32 as char, // Blank
                        fg: Color::RED,    // If you see red you fucked up
                        bg: Color::BLACK,
                    };
                    80 * 50
                ],
                render: true,
            },
            actors: Vec::new(),
            current_actor: 0,
        };
        return s;
    }

    pub fn render(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::BLACK);

        self.root_console.render(d, &self.tilemap);

        for actor in self.actors.iter() {
            actor.render(d, &self.tilemap, self.tilesize);
        }

        d.draw_fps(20, 20);
    }

    pub fn update(&mut self, pressed_key: Option<KeyboardKey>) {
        let mut action = self.actors[self.current_actor].get_action(pressed_key);
        if action.is_none() {
            return;
        }

        loop {
            let result = action
                .as_mut()
                .unwrap()
                .execute(&mut self.actors[self.current_actor]);

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
