use std::ops;

use num::range_step_inclusive;

use crate::{brick::{self, Brick}, grid::GameGrids, op::GameOP, random::{self, RANDOM_SEED, get_random_num}, vec2::{self, Vec2}};

pub const INITIAL_POS: Vec2 = Vec2(4, 0);

#[derive(Clone)]
pub struct GameState {
    pub grids: GameGrids,
    pub score: u32,
    pub rand_num: i32,
    pub ops: Vec<GameOP>,
    pub brick_count: usize,
}

impl GameState {
    pub fn initial_state() -> Self {
        Self {
            grids: GameGrids::new(),
            score: 0,
            rand_num: RANDOM_SEED,
            ops: Vec::with_capacity(10000),
            brick_count: 0,
        }
    }
    pub fn next(&self, next_states: &mut [GameState; 32]) -> usize {
        let next_rand_num = random::get_random_num(self.rand_num);
        let initial_brick = Brick::from_random_num(next_rand_num, self.brick_count);
        let mut next_count = 0;
        for y in (0i8..20).rev() {
            for x in 0i8..10 {
                let pos = Vec2(x, y);
                if self.grids.get(pos) {
                    continue;
                }

                for rot in 0..initial_brick.state_count() {
                    let mut temp_state = self.clone();
                    temp_state.next_brick();
                    let mut brick = initial_brick;
                    if self.find_way(&mut brick, rot, pos, &mut temp_state.ops) {
                        
                        temp_state.grids.place_teris_brick(&brick, pos);
                        temp_state.evaluate_score();
                        next_states[next_count] = temp_state;
                        next_count += 1;
                    }
                }
            }
        }
        next_count
    }

    pub fn find_way(&self, brick: &mut Brick, rotations: usize, pos: Vec2, ops: &mut Vec<GameOP>) -> bool {
        let mut current_pos = INITIAL_POS;
        let diff = pos - current_pos;

        let range_x = match diff.0 {
            x if x < 0 => range_step_inclusive(current_pos.0, pos.0, -1),
            _ => range_step_inclusive(current_pos.0, pos.0, 1),
        };
        for x in range_x {
            current_pos = Vec2(x, current_pos.1);
            if !self.grids.can_place_brick(brick, current_pos) {
                return false;
            }
        }
        for _ in 0..rotations {
            *brick = brick.rotate();
            if !self.grids.can_place_brick(brick, current_pos) {
                return false;
            }
        }
        for y in current_pos.1..pos.1 {
            current_pos.1 = y;
            if !self.grids.can_place_brick(brick, current_pos) {
                return false;
            }
        }
        
        ops.push(GameOP::New);
        match diff.0 {
            x if x < 0 => ops.push(GameOP::Left(i8::abs(diff.0))),
            0 => (),
            _ => ops.push(GameOP::Right(diff.0)),
        };
        if rotations > 0 {
            ops.push(GameOP::Rotate(rotations as i8));
        }
        if diff.1 > 0 {
            ops.push(GameOP::Down(diff.1));
        }
        
        true
    }

    pub fn evaluate_score(&mut self) {
        let mut count = 0;
        let mut occupied_count = 0;
        for row in 0..20 {
            if self.grids.is_full_row(row) {
                count += 1;
                self.grids.clear_row(row);
            } else {
                occupied_count += 1;
            }
        }
        let terris_score = match count {
            1 => occupied_count * 1,
            2 => occupied_count * 3,
            3 => occupied_count * 6,
            4 => occupied_count * 10,
            _ => 0,
        };
        self.score += terris_score;
    }

    pub fn next_brick(&mut self) -> Brick {
        self.rand_num = get_random_num(self.rand_num);
        let brick = Brick::from_random_num(self.rand_num, self.brick_count);
        self.brick_count += 1;

        brick
    }
}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    use crate::game::GameState;

    #[test]
    fn test() {
        // assert_eq!(size_of::<GameState>(), 0);
    }
}