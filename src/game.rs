
use num::range_step_inclusive;

use crate::{brick::{Brick}, grid::GameGrids, op::{GameOP}, random::{self, RANDOM_SEED, get_random_num}, vec2::{Vec2}};

pub const INITIAL_POS: Vec2 = Vec2(4, 0);
pub const MAX_BRICKS_COUNT: usize = 10000;

#[derive(Clone, Default)]
pub struct GameState {
    pub grids: GameGrids,
    pub score: u32,
    pub sp_score: i32,
    pub rand_num: i32,
    pub brick_stack: Vec<u16>,
    pub brick_count: usize,
}

impl GameState {
    pub fn initial_state() -> Self {
        Self {
            grids: GameGrids::new(),
            score: 0,
            sp_score: 0,
            rand_num: RANDOM_SEED,
            brick_stack: Vec::with_capacity(MAX_BRICKS_COUNT),
            brick_count: 0,
        }
    }
    pub fn next(&self, next_states: &mut [GameState]) -> usize {
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

                    let rotated_brick = initial_brick.rotate_n(rot);
                    if !self.grids.can_place_brick(&rotated_brick, pos) {
                        continue;
                    }
                    if rotated_brick.get_top_pos(pos.1) == 0 {
                        continue;
                    }

                    let mut brick = initial_brick;
                    if self.find_way(&mut brick, rot, pos) {
                        let next_state = &mut next_states[next_count];
                        next_state.clone_from(self);
                        next_state.place_brick(&rotated_brick, pos, rot);

                        next_count += 1;
                        if next_count >= next_states.len() {
                            return next_count;
                        }
                    }
                }
            }
        }
        next_count
    }

    pub fn find_way(&self, brick: &mut Brick, rotations: usize, pos: Vec2) -> bool {
        let mut current_pos = INITIAL_POS;
        let diff = pos - current_pos;

        let range_x = match diff.0 {
            x if x < 0 => range_step_inclusive(current_pos.0, pos.0, -1),
            _ => range_step_inclusive(current_pos.0, pos.0, 1),
        };
        for x in range_x {
            current_pos = Vec2(x, current_pos.1);
            if !self.grids.brick_pos_valid(brick, current_pos, true) {
                return false;
            }
        }
        for _ in 0..rotations {
            *brick = brick.rotate();
            if !self.grids.brick_pos_valid(brick, current_pos, true) {
                return false;
            }
        }
        for y in current_pos.1..pos.1 {
            current_pos.1 = y;
            if !self.grids.brick_pos_valid(brick, current_pos, true) {
                return false;
            }
        }
        
        // ops.push(GameOP::New);
        // match diff.0 {
        //     x if x < 0 => ops.push(GameOP::Left(i8::abs(diff.0))),
        //     0 => (),
        //     _ => ops.push(GameOP::Right(diff.0)),
        // };
        // if rotations > 0 {
        //     ops.push(GameOP::Rotate(rotations as i8));
        // }
        // if diff.1 > 0 {
        //     ops.push(GameOP::Down(diff.1));
        // }
        
        true
    }

    pub fn evaluate_score(&mut self) {
        let mut count = 0;
        let mut occupied_blocks_count = 0;
        let mut density = 0f32;
        let mut height = 0;
        let mut top_reached = false;
        for row in 0..20 {
            let blocks = self.grids.blocks_in_row(row);
            occupied_blocks_count += blocks;
            density = density + blocks as f32;
            if !top_reached && self.grids.get_row(row) != 0 {
                top_reached = true;
                height = 20 - row;
            }
            if self.grids.is_full_row(row) {
                count += 1;
                self.grids.remove_row(row);
            }
        }
        density /= height as f32 * 10f32;
        let terris_score = match count {
            1 => occupied_blocks_count * 1,
            2 => occupied_blocks_count * 3,
            3 => occupied_blocks_count * 6,
            4 => occupied_blocks_count * 10,
            _ => 0,
        };
        self.score += terris_score as u32;
        self.sp_score = self.score as i32 
            - (height as i32 - 16).abs() * 1
            - (height as i32 - 17).clamp(0, 10).pow(4) * 12
            + (occupied_blocks_count as i32) * 16
            + (density.powf(1.8) * 200f32) as i32;
    }

    pub fn next_brick(&mut self) -> Brick {
        self.rand_num = get_random_num(self.rand_num);
        let brick = Brick::from_random_num(self.rand_num, self.brick_count);
        self.brick_count += 1;

        brick
    }

    pub fn clone_from(&mut self, state: &Self) {
        self.brick_stack.clear();
        self.grids = state.grids.clone();
        self.brick_count = state.brick_count;
        self.brick_stack.extend_from_slice(&state.brick_stack[..]);
        self.rand_num = state.rand_num;
        self.score = state.score;
        self.sp_score = state.sp_score;
    }

    pub fn place_brick(&mut self, brick: &Brick, pos: Vec2, rot: usize) {
        self.grids.place_teris_brick(brick, pos);
        self.next_brick();
        self.evaluate_score();
        self.brick_stack.push((pos.0 as u16) | (pos.1 as u16) << 4 | (rot as u16) << 10);
    }

    pub fn get_op_sequence(&self) -> Vec<GameOP> {
        let mut ops = Vec::with_capacity(MAX_BRICKS_COUNT * 3);
        let mut ghost = GameState::initial_state();
        for state in &self.brick_stack {
            let pos = Vec2((state & 0b1111) as i8, ((state & 0b111110000) >> 4) as i8);
            let rot = (state & 0b1111_0000000000) >> 10;
            
            ghost.next_brick();
            let diff = pos - INITIAL_POS;
            ops.push(GameOP::New);
            match diff.0 {
                x if x < 0 => ops.push(GameOP::Left(-x)),
                x if x > 0 => ops.push(GameOP::Right(x)),
                _ => (),
            }
            if rot > 0 {
                ops.push(GameOP::Rotate(rot as i8));
            }
            if diff.1 > 0 {
                ops.push(GameOP::Down(diff.1));
            }

        }

        ops
    }
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.sp_score == other.sp_score
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.sp_score.partial_cmp(&other.sp_score)
    }    
}

#[cfg(test)]
mod test {
    use std::mem::size_of;

    use crate::game::GameState;

    #[test]
    fn test() {
        assert!(size_of::<GameState>() * 800000 < 8 * 1024 * 1024 * 1024);
    }
}