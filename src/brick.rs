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
pub mod Bricks {
    pub const I: usize = 0;
    pub const L: usize = 1;
    pub const J: usize = 2;
    pub const T: usize = 3;
    pub const O: usize = 4;
    pub const S: usize = 5;
    pub const Z: usize = 6;
}

pub struct Brick(BrickType, BrickState);

impl Brick {
    pub fn from_random_num(num: usize, brick_count: usize) -> Self {
        let weight_idx = num % 29;
        let state_idx = brick_count % BRICK_STATE_COUNT;
        let shape_idx = match weight_idx {
            0..=1 => Bricks::I,
            2..=4 => Bricks::L,
            5..=7 => Bricks::J,
            8..=11 => Bricks::T,
            12..=16 => Bricks::O,
            17..=22 => Bricks::S,
            23..=28 => Bricks::Z,
            _ => Bricks::I,
        };
        Self(shape_idx, state_idx)
    }

    #[inline(always)]
    pub fn rotate(self) -> Self {
        Self(self.0, self.1 % 4)
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
}
