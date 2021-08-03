
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

fn main() {
    let game = Game::new();
    game.start();
}
