use crate::{brick::Brick, game::{self, GameState}, game_io::{self, GetInput, RenderGame}, op::GameOP, vec2::{Vec2}};
use crate::game_io::GameRenderer;

pub struct Game {
    state: GameState,
    brick_pos: Vec2,
    brick: Brick,
    renderer: GameRenderer,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::initial_state(),
            brick: Brick::from_random_num(0, 0),
            brick_pos: game::INITIAL_POS,
            renderer: GameRenderer::new(),
        }
    }

    pub fn start(mut self) {
        let mut input = game_io::GameInput::new();
        self.render();
        
        loop {
            let op = input.get_input();
            if !self.update(op) {
                
                self.render();
                println!("Game Over!");
                return;
            }
            
            self.render();
        }
    }

    pub fn update(&mut self, op: GameOP) -> bool
    {
        
        let next_pos = match op {
            GameOP::New => {
                game::INITIAL_POS
            },
            GameOP::Rotate(_) => {
                self.brick_pos
            },
            GameOP::Left(dx) => self.brick_pos + Vec2(-dx, 0),
            GameOP::Right(dx) => self.brick_pos + Vec2(dx, 0),
            GameOP::Down(dy) => self.brick_pos + Vec2(0, dy),
        };

        if self.state.grids.brick_pos_valid(&self.brick, next_pos, true) {
            match op {
                GameOP::New => {
                    self.state.grids.place_teris_brick(&self.brick, self.brick_pos);
                    self.state.evaluate_score();
                    let next_brick = self.state.next_brick();
                    if !self.state.grids.brick_pos_valid(&next_brick, game::INITIAL_POS, true) {
                        return false;
                    }
                    self.brick = next_brick;
                },
                GameOP::Rotate(rot) => {
                    let mut brick = self.brick;
                    for _ in 0..rot {
                        brick = brick.rotate();
                    }
                    if !self.state.grids.brick_pos_valid(&brick, self.brick_pos, true) {
                        return true;
                    }
                    self.brick = brick;
                }
                _ =>(),
            }
            self.brick_pos = next_pos;
        }

        true
    }


    pub fn render(&mut self) {
        self.renderer.render_game(&self.state);
        self.renderer.render_user_hint();
        self.renderer.render_brick(&self.brick, self.brick_pos);
    }

}
