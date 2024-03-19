use std::{arch::x86_64::*, mem, simd::*};

const BB_WIDTH: usize = 8;
const BB_HEIGHT: usize = 16;

pub struct BoardBits {
    m: __m128i,
}

// TODO: use trait to make this reusable
impl BoardBits {
    fn get(&self, x: usize, y: usize) -> bool {
        debug_assert!(Self::within_bound(x, y));

        unsafe { _mm_testz_si128(Self::onebit(x, y), self.m) == 0 }
    }
}

impl BoardBits {
    const fn within_bound(x: usize, y: usize) -> bool {
        x < BB_WIDTH && y < BB_HEIGHT
    }

    const fn onebit(x: usize, y: usize) -> __m128i {
        debug_assert!(Self::within_bound(x, y));

        let shift = ((x << 4) | y) & 0x3F; // x << 4: choose column by multiplying 16
        let hi = (x as u64) >> 2; // 1 if x >= 4 else 0
        let lo = hi ^ 1;
        unsafe { mem::transmute(u64x2::from_array([lo << shift, hi << shift])) }
    }

    const fn board_mask_12() -> Self {
        Self {
            m: unsafe {
                mem::transmute(u16x8::from_array([
                    0x0000, 0x1FFE, 0x1FFE, 0x1FFE, 0x1FFE, 0x1FFE, 0x1FFE, 0x0000,
                ]))
            },
        }
    }

    const fn board_mask_13() -> Self {
        Self {
            m: unsafe {
                mem::transmute(u16x8::from_array([
                    0x0000, 0x3FFE, 0x3FFE, 0x3FFE, 0x3FFE, 0x3FFE, 0x3FFE, 0x0000,
                ]))
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{HEIGHT, WIDTH};

    #[test]
    fn board_mask() {
        let mask_12 = BoardBits::board_mask_12();
        let mask_13 = BoardBits::board_mask_13();

        for x in 0..BB_WIDTH {
            for y in 0..BB_HEIGHT {
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
