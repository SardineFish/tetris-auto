use std::{collections::HashMap};

use crate::vec2::Vec2;

const BRICK_STATE_COUNT: usize = 4;

const BRICKS_CONFIG: [[[Vec2; 4]; 4]; 7] = [
    [
        // I 型
        [Vec2(0, 0), Vec2(0, -1), Vec2(0, -2), Vec2(0, 1)],
        [Vec2(0, 0), Vec2(1, 0), Vec2(2, 0), Vec2(-1, 0)],
        [Vec2(0, 0), Vec2(0, -1), Vec2(0, -2), Vec2(0, 1)],
        [Vec2(0, 0), Vec2(1, 0), Vec2(2, 0), Vec2(-1, 0)],
    ],
    [
        // L 型
        [Vec2(0, 0), Vec2(0, -1), Vec2(0, -2), Vec2(1, 0)],
        [Vec2(0, 0), Vec2(1, 0), Vec2(2, 0), Vec2(0, 1)],
        [Vec2(0, 0), Vec2(-1, 0), Vec2(0, 1), Vec2(0, 2)],
        [Vec2(0, 0), Vec2(0, -1), Vec2(-1, 0), Vec2(-2, 0)],
    ],
    [
        // J 型
        [Vec2(0, 0), Vec2(0, -1), Vec2(0, -2), Vec2(-1, 0)],
        [Vec2(0, 0), Vec2(0, -1), Vec2(1, 0), Vec2(2, 0)],
        [Vec2(0, 0), Vec2(1, 0), Vec2(0, 1), Vec2(0, 2)],
        [Vec2(0, 0), Vec2(-1, 0), Vec2(-2, 0), Vec2(0, 1)],
    ],
    [
        // T 型
        [Vec2(0, 0), Vec2(1, 0), Vec2(0, 1), Vec2(-1, 0)],
        [Vec2(0, 0), Vec2(0, -1), Vec2(0, 1), Vec2(-1, 0)],
        [Vec2(0, 0), Vec2(0, -1), Vec2(1, 0), Vec2(-1, 0)],
        [Vec2(0, 0), Vec2(0, -1), Vec2(1, 0), Vec2(0, 1)],
    ],
    [
        // O 型
        [Vec2(0, 0), Vec2(0, -1), Vec2(1, -1), Vec2(1, 0)],
        [Vec2(0, 0), Vec2(0, -1), Vec2(1, -1), Vec2(1, 0)],
        [Vec2(0, 0), Vec2(0, -1), Vec2(1, -1), Vec2(1, 0)],
        [Vec2(0, 0), Vec2(0, -1), Vec2(1, -1), Vec2(1, 0)],
    ],
    [
        // S 型
        [Vec2(0, 0), Vec2(0, -1), Vec2(1, -1), Vec2(-1, 0)],
        [Vec2(0, 0), Vec2(-1, 0), Vec2(-1, -1), Vec2(0, 1)],
        [Vec2(0, 0), Vec2(0, -1), Vec2(1, -1), Vec2(-1, 0)],
        [Vec2(0, 0), Vec2(-1, 0), Vec2(-1, -1), Vec2(0, 1)],
    ],
    [
        // Z 型
        [Vec2(0, 0), Vec2(0, -1), Vec2(1, 0), Vec2(-1, -1)],
        [Vec2(0, 0), Vec2(0, -1), Vec2(-1, 1), Vec2(-1, 0)],
        [Vec2(0, 0), Vec2(0, -1), Vec2(1, 0), Vec2(-1, -1)],
        [Vec2(0, 0), Vec2(0, -1), Vec2(-1, 1), Vec2(-1, 0)],
    ],
];

type BrickType = usize;
type BrickState = usize;

#[allow(non_snake_case)]
pub mod shapes {
    pub const I: usize = 0;
    pub const L: usize = 1;
    pub const J: usize = 2;
    pub const T: usize = 3;
    pub const O: usize = 4;
    pub const S: usize = 5;
    pub const Z: usize = 6;
}

lazy_static::lazy_static!{
    pub static ref BRICKS_BOTTOM: [[Vec<Vec2>; 4]; 7] = {
        let mut bricks_bottom: [[Vec<Vec2>; 4]; 7] = Default::default();

        for shape in 0..7 {
            for state in 0..4 {
                let mut bottom_pos = HashMap::<i8, i8>::new();
                for pos in &BRICKS_CONFIG[shape][state] {
                    match bottom_pos.get_mut(&pos.0) {
                        Some(y) => if pos.1 > *y {
                            *y = pos.1;
                        },
                        None => {
                            bottom_pos.insert(pos.0, pos.1);
                        }
                    }
                }
                for (x, y) in bottom_pos {
                    bricks_bottom[shape][state].push(Vec2(x, y + 1));
                }
            }
        }

        bricks_bottom
    };

    pub static ref BRICKS_TOP: [[i8; 4]; 7] = {
        let mut bricks_top: [[i8; 4]; 7] = Default::default();

        for shape in 0..7 {
            for state in 0..4 {
                let mut min_y = i8::MAX;
                for pos in &BRICKS_CONFIG[shape][state] {
                    if pos.1 < min_y {
                        min_y = pos.1;
                    }
                }
                bricks_top[shape][state] = min_y;
            }
        }

        bricks_top
    };
}

#[derive(Clone, Copy, Default)]
pub struct Brick(pub BrickType, pub BrickState);

impl Brick {
    pub fn from_random_num(num: i32, brick_count: usize) -> Self {
        let weight_idx = num % 29;
        let state_idx = brick_count % BRICK_STATE_COUNT;
        let shape_idx = match weight_idx {
            0..=1 => shapes::I,
            2..=4 => shapes::L,
            5..=7 => shapes::J,
            8..=11 => shapes::T,
            12..=16 => shapes::O,
            17..=22 => shapes::S,
            23..=28 => shapes::Z,
            _ => shapes::I,
        };
        Self(shape_idx, state_idx)
    }

    #[inline(always)]
    pub fn rotate(self) -> Self {
        Self(self.0, (self.1 + 1) % 4)
    }

    #[inline(always)]
    pub fn rotate_n(self, rot: usize) -> Self {
        Self(self.0, (self.1 + rot) % 4)
    }

    #[inline(always)]
    pub fn state_count(&self) -> usize {
        match self.0 {
            shapes::O => 1,
            shapes::I | shapes::S | shapes::Z => 2,
            _ => 4,
        }
    }

    #[inline(always)]
    pub fn inverse_rotate(self) -> Self {
        Self(self.0, (self.1 + 3) % 4)
    }

    #[inline(always)]
    pub fn get_pos(&self) -> &'static [Vec2; 4] {
        &BRICKS_CONFIG[self.0][self.1]
    }

    #[inline(always)]
    pub fn pos_with_center(&self, center: Vec2) -> [Vec2; 4] {
        [
            BRICKS_CONFIG[self.0][self.1][0] + center,
            BRICKS_CONFIG[self.0][self.1][1] + center,
            BRICKS_CONFIG[self.0][self.1][2] + center,
            BRICKS_CONFIG[self.0][self.1][3] + center,
        ]
    }

    #[inline(always)]
    pub fn get_lower_bound(&self) -> &'static Vec<Vec2> {
        &BRICKS_BOTTOM[self.0][self.1]
    }

    #[inline(always)]
    pub fn get_top_pos(&self, y: i8) -> i8 {
        BRICKS_TOP[self.0][self.1] + y
    } 
}
