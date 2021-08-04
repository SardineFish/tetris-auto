use std::ops;

use num::range_step_inclusive;
use termion::{clear, cursor};

use crate::{brick::{self, Brick}, grid::GameGrids, op::GameOP, random::{self, RANDOM_SEED, get_random_num}, vec2::{self, Vec2}};

pub const INITIAL_POS: Vec2 = Vec2(4, 0);

#[derive(Clone, Default)]
pub struct GameState {
    pub grids: GameGrids,
    pub score: u32,
    pub sp_score: i32,
    pub rand_num: i32,
    pub ops: Vec<GameOP>,
    pub brick_count: usize,
}

impl GameState {
    pub fn initial_state() -> Self {
        Self {
            grids: GameGrids::new(),
            score: 0,
            sp_score: 0,
            rand_num: RANDOM_SEED,
            ops: Vec::with_capacity(10000),
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

                    let mut temp_state = self.clone();
                    temp_state.next_brick();
                    let mut brick = initial_brick;
                    if self.find_way(&mut brick, rot, pos, &mut temp_state.ops) {
                        
                        temp_state.grids.place_teris_brick(&brick, pos);
                        temp_state.evaluate_score();
                        next_states[next_count] = temp_state;
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

    pub fn find_way(&self, brick: &mut Brick, rotations: usize, pos: Vec2, ops: &mut Vec<GameOP>) -> bool {
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
            - (height as i32 - 14).abs() / 2
            + (occupied_blocks_count as i32) * 3
            + (density * 10f32) as i32;
    }

    pub fn next_brick(&mut self) -> Brick {
        self.rand_num = get_random_num(self.rand_num);
        let brick = Brick::from_random_num(self.rand_num, self.brick_count);
        self.brick_count += 1;

        brick
    }

    pub fn render(&self) {
        print!("{}", clear::All);
        print!("{}Score: {}", cursor::Goto(13, 3), self.score);
        print!("{}Bricks: {}", cursor::Goto(13, 5), self.brick_count);
        for y in 0..20 {
            for x in 0..10 {
                match self.grids.get(Vec2(x, y)) {
                    true => print!("{}*", cursor::Goto(x as u16 + 1, y as u16 + 1)),
                    false => print!("{} ", cursor::Goto(x as u16 + 1, y as u16 + 1)),
                }
            }
        }
        for y in 1..=20 {
            print!("{}|", cursor::Goto(11, y));
        }
        for x in 1..=10 {
            print!("{}-", cursor::Goto(x, 21));
        }
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
        // assert_eq!(size_of::<GameState>(), 0);
    }
}