use actions::{movement::WalkAction, Action};
use actors::Actor;
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
    };

    gamestate.actors.push(test_actor);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        gamestate.update();
        gamestate.render(&mut d);
    }
}

pub struct GameState {
    pub tilemap: Texture2D,
    pub tilesize: i32,
    pub width: i32,
    pub height: i32,
    pub root_console: Console,
    pub actors: Vec<Actor>,
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

    pub fn update(&mut self) {
        for actor in self.actors.iter_mut() {
            let wa: WalkAction = WalkAction { x: 1, y: 0 };
            wa.execute(actor);
        }
    }
}
