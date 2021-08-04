
use std::ops::Shl;

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

#[derive(Clone, Default)]
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
    pub fn is_empty(&self, pos: Vec2) -> bool {
        !self.get(pos)
    }

    #[inline(always)]
    pub fn set(&mut self, pos: Vec2, val: u64)  {
        let (nint, nbit) = Self::pos_to_idx(pos);
        self.bits[nint as usize] |= val << nbit;
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
    pub fn can_place_brick(&self, brick: &Brick, center: Vec2) -> bool {
        if !self.brick_pos_valid(brick, center, false) {
            return false;
        }
        let lower_bound = brick.get_lower_bound();
        let mut can_place = false;
        for bound_pos in lower_bound {
            let pos = center + *bound_pos;
            
            can_place = can_place || pos.1 == 20 || self.get(pos);
        }
        can_place
    }

    #[inline(always)]
    pub fn brick_pos_valid(&self, brick: &Brick, center: Vec2, allow_outbound: bool) -> bool {
        let brick_pos = brick.get_pos();
        for i in 0..4 {
            let pos = brick_pos[i] + center;
            match pos {
                Vec2(0..=9, 0..=19) => (),
                Vec2(0..=9, y) if y < 20 => match allow_outbound {
                    true => continue,
                    false => return false,
                },
                _ => return false,
            }
            if self.get(brick_pos[i] + center) {
                return false;
            }
        }

        true
    }

    // #[inline(always)]
    pub fn get_row(& self, y: i8) -> u64 {
        let (nint, mask) = Self::row_mask(y);
        let nseg = Self::pos_to_nseg(y);
        (self.bits[nint] & mask) >> (nseg * 16)
    }

    // #[inline(always)]
    pub fn is_full_row(&self, y: i8) -> bool {
        let row = self.get_row(y);
        (row + 1) & (1 << 10) != 0
    }

    pub fn blocks_in_row(&self, y: i8) -> usize {
        let row = self.get_row(y);
        10 - zeros_in_num(row, 10)
    }

    #[inline]
    pub fn remove_row(&mut self, y: i8) -> u64 {
        let (nint, mask) = Self::row_mask(y);
        let nseg = Self::pos_to_nseg(y);
        let rowbits = (self.bits[nint] & mask) >> (nseg * 16);

        let higher_mask = match nseg {
            3 => 0,
            _ => u64::MAX << ((nseg as u64 + 1) * 16) 
        };
        self.bits[nint] = (self.bits[nint] & higher_mask) | ((self.bits[nint] << 16) & (!higher_mask));
        for nint in (0..nint).rev() {
            self.bits[nint] = self.bits[nint].rotate_left(16);
            self.bits[nint + 1] = (self.bits[nint + 1] & !0xFFFF) | (self.bits[nint] & 0xFFFF);
        }
        self.bits[0] &= 0xFFFF_FFFF_FFFF_0000;
        rowbits
    }
}

fn zeros_in_num(mut x: u64, max_pos: usize) -> usize {
    let mut mask = 1;
    let mut count = 0;
    loop {
        mask = (x + mask) ^ x;
        if mask >= 1 << max_pos {
            break;
        }
        count += 1;
        x |= mask;
    }
    count
}

#[cfg(test)]
mod test {
    use crate::grid::zeros_in_num;

    #[test]
    fn test_zeros_in_num() {
        assert_eq!(zeros_in_num(0b11_1111_1111, 10), 0);
        assert_eq!(zeros_in_num(0b11_1111_1110, 10), 1);
        assert_eq!(zeros_in_num(0b01_1110_1110, 10), 3);
        assert_eq!(zeros_in_num(0b11_1000_1111, 10), 3);
        assert_eq!(zeros_in_num(0b00_0000_0000, 10), 10);
        assert_eq!(zeros_in_num(0b01_1001_0110, 10), 5);
        assert_eq!(zeros_in_num(0b11100_1111_1111, 10), 2);
        assert_eq!(zeros_in_num(0b00_1111_1111, 10), 2);
    }
}