use std::{arch::x86_64::*, mem, simd::*};

use crate::board::{ENTIRE_HEIGHT, ENTIRE_WIDTH};

pub struct BoardBits(__m128i);

// TODO: define trait to make this reusable
impl BoardBits {
    pub fn zero() -> Self {
        unsafe { Self(_mm_setzero_si128()) }
    }

    // TODO: can we make trait members const fn?
    pub fn wall() -> Self {
        unsafe {
            mem::transmute(u16x8::from_array([
                0xFFFF, 0x8001, 0x8001, 0x8001, 0x8001, 0x8001, 0x8001, 0xFFFF,
            ]))
        }
    }

    pub fn onebit(x: usize, y: usize) -> Self {
        debug_assert!(Self::within_bound(x, y));

        let shift = ((x << 4) | y) & 0x3F; // x << 4: choose column by multiplying 16
        let hi = (x as u64) >> 2; // 1 if x >= 4 else 0
        let lo = hi ^ 1;

        unsafe { mem::transmute(u64x2::from_array([lo << shift, hi << shift])) }
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        debug_assert!(Self::within_bound(x, y));

        unsafe { _mm_testz_si128(Self::onebit(x, y).0, self.0) == 0 }
    }
}

impl BoardBits {
    const fn within_bound(x: usize, y: usize) -> bool {
        x < ENTIRE_WIDTH && y < ENTIRE_HEIGHT
    }

    const unsafe fn board_mask_12() -> Self {
        mem::transmute(u16x8::from_array([
            0x0000, 0x1FFE, 0x1FFE, 0x1FFE, 0x1FFE, 0x1FFE, 0x1FFE, 0x0000,
        ]))
    }

    const unsafe fn board_mask_13() -> Self {
        mem::transmute(u16x8::from_array([
            0x0000, 0x3FFE, 0x3FFE, 0x3FFE, 0x3FFE, 0x3FFE, 0x3FFE, 0x0000,
        ]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{HEIGHT, WIDTH};

    #[test]
    fn wall() {
        let wall = BoardBits::wall();

        for x in 0..ENTIRE_WIDTH {
            for y in 0..ENTIRE_HEIGHT {
                assert_eq!(
                    wall.get(x, y),
                    x == 0 || x == ENTIRE_WIDTH - 1 || y == 0 || y == ENTIRE_HEIGHT - 1,
                    "wall is incorrect at (x: {}, y: {})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn board_mask() {
        // TODO: replace with public (safe) fn
        let mask_12 = unsafe { BoardBits::board_mask_12() };
        let mask_13 = unsafe { BoardBits::board_mask_13() };

        for x in 0..ENTIRE_WIDTH {
            for y in 0..ENTIRE_HEIGHT {
                assert_eq!(
                    mask_12.get(x, y),
                    1 <= x && x <= WIDTH && 1 <= y && y <= HEIGHT,
                    "mask_12 is incorrect at (x: {}, y: {})",
                    x,
                    y
                );
                assert_eq!(
                    mask_13.get(x, y),
                    1 <= x && x <= WIDTH && 1 <= y && y <= HEIGHT + 1,
                    "mask_13 is incorrect at (x: {}, y: {})",
                    x,
                    y
                );
            }
        }
    }
}
