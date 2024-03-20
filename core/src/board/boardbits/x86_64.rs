use std::{arch::x86_64::*, mem, simd::*};

use super::BoardManipulation;
use crate::board::{ENTIRE_HEIGHT, ENTIRE_WIDTH};

#[repr(align(16))]
#[derive(Clone, Copy)]
pub struct BoardBits(__m128i);

impl BoardManipulation for BoardBits {
    fn zero() -> Self {
        unsafe { Self(_mm_setzero_si128()) }
    }

    // TODO: can we make trait members const fn?
    fn wall() -> Self {
        unsafe {
            mem::transmute(u16x8::from_array([
                0xFFFF, 0x8001, 0x8001, 0x8001, 0x8001, 0x8001, 0x8001, 0xFFFF,
            ]))
        }
    }

    fn onebit(x: usize, y: usize) -> Self {
        debug_assert!(Self::within_bound(x, y));

        let shift = ((x << 4) | y) & 0x3F; // x << 4: choose column by multiplying 16
        let hi = (x as u64) >> 2; // 1 if x >= 4 else 0
        let lo = hi ^ 1;

        unsafe { mem::transmute(u64x2::from_array([lo << shift, hi << shift])) }
    }

    fn mask(&self, mask: Self) -> Self {
        *self & mask
    }

    fn mask_12(&self) -> Self {
        self.mask(unsafe { Self::board_mask_12() })
    }

    fn mask_13(&self) -> Self {
        self.mask(unsafe { Self::board_mask_13() })
    }

    fn not_mask(&self, mask: Self) -> Self {
        unsafe { mask.andnot(*self) }
    }

    fn not_mask_12(&self) -> Self {
        self.not_mask(unsafe { Self::board_mask_12() })
    }

    fn not_mask_13(&self) -> Self {
        self.not_mask(unsafe { Self::board_mask_13() })
    }

    fn get(&self, x: usize, y: usize) -> bool {
        debug_assert!(Self::within_bound(x, y));

        unsafe { _mm_testz_si128(Self::onebit(x, y).0, self.0) == 0 }
    }
}

impl std::ops::BitAnd for BoardBits {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}

impl BoardBits {
    const fn within_bound(x: usize, y: usize) -> bool {
        x < ENTIRE_WIDTH && y < ENTIRE_HEIGHT
    }

    /// Calculate `(~a) & b`.
    unsafe fn andnot(&self, rhs: Self) -> Self {
        Self(_mm_andnot_si128(self.0, rhs.0))
    }

    const unsafe fn board_mask_full() -> Self {
        mem::transmute(u16x8::from_array([
            0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        ]))
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
    fn mask() {
        let mask_full = unsafe { BoardBits::board_mask_full() };
        let mask_12 = mask_full.mask_12();
        let mask_13 = mask_full.mask_13();

        for x in 0..ENTIRE_WIDTH {
            for y in 0..ENTIRE_HEIGHT {
                assert!(
                    mask_full.get(x, y),
                    "mask_full is incorrect at (x: {}, y: {})",
                    x,
                    y
                );
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

    #[test]
    fn not_mask() {
        let mask_full = unsafe { BoardBits::board_mask_full() };
        let not_mask_12 = mask_full.not_mask_12();
        let not_mask_13 = mask_full.not_mask_13();

        for x in 0..ENTIRE_WIDTH {
            for y in 0..ENTIRE_HEIGHT {
                assert!(
                    mask_full.get(x, y),
                    "mask_full is incorrect at (x: {}, y: {})",
                    x,
                    y
                );
                assert_eq!(
                    not_mask_12.get(x, y),
                    x < 1 || x > WIDTH || y < 1 || y > HEIGHT,
                    "not_mask_12 is incorrect at (x: {}, y: {})",
                    x,
                    y
                );
                assert_eq!(
                    not_mask_13.get(x, y),
                    x < 1 || x > WIDTH || y < 1 || y > HEIGHT + 1,
                    "not_mask_13 is incorrect at (x: {}, y: {})",
                    x,
                    y
                );
            }
        }
    }
}
