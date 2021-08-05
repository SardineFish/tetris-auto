

use auto::TetrisAuto;
use game_io::GetInput;
use game_play::Game;

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
pub mod game_io;

#[allow(warnings)]
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

    let (mut kill_bus, join) = TetrisAuto::run_continuous(8);
    let mut input = game_io::GameInput::new();
    loop {
        if input.try_get_interrupt().is_ok() {
            kill_bus.broadcast(());
            break;
        }
    }
    join.join();
}
