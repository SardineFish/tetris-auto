
use crate::{brick::Brick, vec2::Vec2};

pub const GRID_WIDTH: u32 = 10;
pub const GRID_HEIGHT: u32 = 20;

/*

0000_0000_0000_0000 0000_0000_0000_0000 0000_0000_0000_0000 0000_0000_0000_0000
0000_0000_0000_0000 0000_0000_0000_0000 0000_0000_0000_0000 0000_0000_0000_0000
0000_0000_0000_0000 0000_0000_0000_0000 0000_0000_0000_0000 0000_0000_0000_0000
0000_0000_0000_0000 0000_0000_0000_0000 0000_0000_0000_0000 0000_0000_0000_0000
0000_0000_0000_0000 0000_0000_0000_0000 0000_0000_0000_0000 0000_0000_0000_0000    

*/

pub struct GameGrids {
    bits: [u64; 5],
}

impl GameGrids {
    pub fn new()-> Self {
        Self {
            bits: [0; 5],
        }
    }

    #[inline(always)]
    fn pos_to_nint(y: i8) -> usize {
        y as usize / 4
    }

    #[inline(always)]
    fn pos_to_nseg(y: i8) -> usize {
        y as usize % 4
    }

    #[inline(always)]
    fn pos_to_nbit(pos: Vec2) -> usize {
        Self::pos_to_nseg(pos.1) * 16 + pos.0 as usize
    }

    #[inline(always)]
    fn pos_to_idx(pos: Vec2) -> (usize, usize) {
        (Self::pos_to_nint(pos.1), Self::pos_to_nbit(pos))
    }

    #[inline(always)]
    fn row_mask(y: i8) -> (usize, u64) {
        (Self::pos_to_nint(y), 0b1111_1111_11 << (Self::pos_to_nseg(y) * 16))
    }

    #[inline(always)]
    fn column_mask(x: i8) -> u64 {
        0x1000100010001 << x
    }

    #[inline(always)]
    pub fn get(&self, pos: Vec2) -> bool {
        let (nint, nbit) = Self::pos_to_idx(pos);
        (self.bits[nint as usize] & (1 << nbit)) != 0
    }

    #[inline(always)]
    pub fn set(&mut self, pos: Vec2, val: u64)  {
        let (nint, nbit) = Self::pos_to_idx(pos);
        self.bits[nint as usize] &= val << nbit;
    }

    #[inline(always)]
    pub fn set_block(&mut self, pos: Vec2) {
        self.set(pos, 1)
    }

    #[inline(always)]
    pub fn set_empty(&mut self, pos: Vec2) {
        self.set(pos, 0)
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.bits.fill(0)
    }

    #[inline(always)]
    pub fn place_teris_brick(&mut self, brick: &Brick, center: Vec2) {
        let brick_pos = brick.get_pos();
        for i in 0..4 {
            self.set_block(brick_pos[i] + center);
        }
    }
    
    #[inline(always)]
    pub fn check_pos_valid(&mut self, brick: &Brick, center: Vec2) -> bool {
        let brick_pos = brick.get_pos();
        for i in 0..4 {
            if self.get(brick_pos[i] + center) {
                return false;
            }
        }
        return true;
    }

    #[inline(always)]
    pub fn get_row(& self, y: i8) -> u64 {
        let (nint, mask) = Self::row_mask(y);
        let nseg = Self::pos_to_nseg(y);
        (self.bits[nint] & mask) >> nseg
    }

    #[inline(always)]
    pub fn is_full_line(&self, y: i8) -> bool {
        let row = self.get_row(y);
        (row + 1) & (1 << 10) != 0
    }
}