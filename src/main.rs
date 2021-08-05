
use std::{io::{self, stdin}, process, thread};

use auto::TetrisAuto;
use game_play::Game;
use termion::{event::Key, input::TermRead};

pub mod grid;
pub mod game;
pub mod brick;
pub mod vec2;
pub mod random;
pub mod op;
pub mod game_play;
pub mod auto;
pub mod fixed_heap;
pub mod utils;

fn main() {
    let game = Game::new();
    // game.start();

    // thread::spawn(|| {
    //     let keys = stdin().keys();
    //     for key in keys {
    //         match key {
    //             Ok(Key::Ctrl('c')) => process::exit(0),
    //             _ => (),
    //         }
    //     }
    // });

    TetrisAuto::start();
}
