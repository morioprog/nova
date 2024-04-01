use super::BoardOps;
use crate::board::{ENTIRE_HEIGHT, ENTIRE_WIDTH};

#[repr(align(16))]
#[derive(Clone, Copy)]
pub struct BoardBits(u128);

impl BoardOps for BoardBits {
    fn zero() -> Self {
        Self(0u128)
    }

    fn wall() -> Self {
        Self(0xFFFF_8001_8001_8001_8001_8001_8001_FFFF)
    }

    fn board_mask_12() -> Self {
        Self(0x0000_1FFE_1FFE_1FFE_1FFE_1FFE_1FFE_0000)
    }

    fn board_mask_13() -> Self {
        Self(0x0000_3FFE_3FFE_3FFE_3FFE_3FFE_3FFE_0000)
    }

    fn full_mask() -> Self {
        Self(0xFFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF)
    }

    fn onebit(x: usize, y: usize) -> Self {
        debug_assert!(Self::within_bound(x, y));

        let shift = ((x << 4) | y) & 0x3F; // x << 4: choose column by multiplying 16
        let hi = (x as u128) >> 2; // 1 if x >= 4 else 0
        let lo = hi ^ 1;

        Self(hi << (shift + 64) | lo << shift)
    }

    fn andnot(&self, rhs: Self) -> Self {
        Self(!self.0 & rhs.0)
    }

    fn shift_up(&self) -> Self {
        Self((self.0 << 1) & 0xFFFE_FFFE_FFFE_FFFE_FFFE_FFFE_FFFE_FFFE)
    }

    fn shift_down(&self) -> Self {
        Self((self.0 >> 1) & 0x7FFF_7FFF_7FFF_7FFF_7FFF_7FFF_7FFF_7FFF)
    }

    fn shift_left(&self) -> Self {
        Self(self.0 >> ENTIRE_HEIGHT)
    }

    fn shift_right(&self) -> Self {
        Self(self.0 << ENTIRE_HEIGHT)
    }

    fn popcount(&self) -> usize {
        self.0.count_ones() as usize
    }

    fn lsb(&self) -> Self {
        Self(1u128 << self.0.trailing_zeros())
    }

    fn max_u16x8(&self) -> u16 {
        let arr: [u16; 8] = (*self).into();
        *arr.iter().max().unwrap()
    }

    fn popcount_u16x8(&self) -> Self {
        self.popcount_u16x8_array().into()
    }

    fn popcount_u16x8_array(&self) -> [u16; 8] {
        let arr: [u16; 8] = (*self).into();
        arr.iter()
            .map(|i| i.count_ones() as u16)
            .collect::<Vec<u16>>()
            .try_into()
            .unwrap()
    }

    fn set_below_top_one_u16x8(&self) -> Self {
        let mut b = self.0;
        b = b | ((b >> 1) & 0x7FFF_7FFF_7FFF_7FFF_7FFF_7FFF_7FFF_7FFF);
        b = b | ((b >> 2) & 0x3FFF_3FFF_3FFF_3FFF_3FFF_3FFF_3FFF_3FFF);
        b = b | ((b >> 4) & 0x0FFF_0FFF_0FFF_0FFF_0FFF_0FFF_0FFF_0FFF);
        b = b | ((b >> 8) & 0x00FF_00FF_00FF_00FF_00FF_00FF_00FF_00FF);
        Self(b)
    }

    fn pext_u64(a: u64, mut mask: u64) -> u64 {
        let mut b = a & mask;
        let mut mk = !mask << 1;
        for i in 0..6 {
            let mp = Self::suffix_parity_u64(mk);
            let mv = mp & mask;
            mask = (mask ^ mv) | (mv >> (1 << i));
            let t = b & mv;
            b = (b ^ t) | (t >> (1 << i));
            mk &= !mp;
        }
        b
    }

    fn pdep_u64(mut a: u64, mask: u64) -> u64 {
        let mut m = mask;
        let mut mk = !m << 1;
        let mut mvs = [0; 6];

        for i in 0..6 {
            let mp = Self::suffix_parity_u64(mk);
            let mv = mp & m;
            m = (m ^ mv) | (mv >> (1 << i));
            mk &= !mp;
            mvs[i] = mv;
        }

        for i in (0..6).rev() {
            let mv = mvs[i];
            let t = a << (1 << i);
            a = (a & !mv) | (t & mv);
        }

        a & mask
    }

    fn before_pop_mask(popped: Self) -> (u64, u64) {
        (popped ^ Self::full_mask()).into()
    }

    fn after_pop_mask(popped: Self) -> (u64, u64) {
        let ppc = popped.popcount_u16x8_array();
        let b: BoardBits = ppc.map(|i| 0xFFFF >> i).into();
        b.into()
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        debug_assert!(Self::within_bound(x, y));

        (Self::onebit(x, y).0 & self.0 != 0) as u8
    }

    fn set_0(&mut self, x: usize, y: usize) {
        debug_assert!(Self::within_bound(x, y));

        self.0 = Self::onebit(x, y).andnot(*self).0
    }

    fn set_1(&mut self, x: usize, y: usize) {
        debug_assert!(Self::within_bound(x, y));

        self.0 = (*self | Self::onebit(x, y)).0
    }

    fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

impl PartialEq<Self> for BoardBits {
    fn eq(&self, other: &Self) -> bool {
        self.0 ^ other.0 == 0
    }
}

impl std::ops::BitAnd for BoardBits {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl std::ops::BitOr for BoardBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitXor for BoardBits {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl From<(u64, u64)> for BoardBits {
    fn from(value: (u64, u64)) -> Self {
        let lo = value.0 as u128;
        let hi = value.1 as u128;
        Self(hi << 64 | lo)
    }
}

impl From<[u16; 8]> for BoardBits {
    fn from(value: [u16; 8]) -> Self {
        let mut b = 0;
        for i in value.iter().rev() {
            b <<= 16;
            b |= *i as u128;
        }
        Self(b)
    }
}

impl From<BoardBits> for (u64, u64) {
    fn from(value: BoardBits) -> Self {
        let lo = (value.0 & 0xFFFF_FFFF_FFFF_FFFF) as u64;
        let hi = (value.0 >> 64) as u64;
        (lo, hi)
    }
}

impl From<BoardBits> for [u16; 8] {
    fn from(value: BoardBits) -> Self {
        let mut b = value.0;
        let mut arr = [0; 8];
        for i in 0..8 {
            arr[i] = (b & 0xFFFF) as u16;
            b >>= 16;
        }
        arr
    }
}

impl BoardBits {
    const fn within_bound(x: usize, y: usize) -> bool {
        x < ENTIRE_WIDTH && y < ENTIRE_HEIGHT
    }

    fn suffix_parity_u64(mut x: u64) -> u64 {
        for i in 0..6 {
            x ^= x << (1 << i);
        }
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{HEIGHT, WIDTH};

    #[test]
    fn constructors() {
        let zero = BoardBits::zero();
        let wall = BoardBits::wall();
        let mask_12 = BoardBits::board_mask_12();
        let mask_13 = BoardBits::board_mask_13();
        let full_mask = BoardBits::full_mask();

        for x in 0..ENTIRE_WIDTH {
            for y in 0..ENTIRE_HEIGHT {
                assert!(
                    zero.get(x, y) == 0,
                    "zero is incorrect at (x: {}, y: {})",
                    x,
                    y
                );
                assert_eq!(
                    wall.get(x, y) == 1,
                    x == 0 || x == ENTIRE_WIDTH - 1 || y == 0 || y == ENTIRE_HEIGHT - 1,
                    "wall is incorrect at (x: {}, y: {})",
                    x,
                    y,
                );
                assert_eq!(
                    mask_12.get(x, y) == 1,
                    1 <= x && x <= WIDTH && 1 <= y && y <= HEIGHT,
                    "mask_12 is incorrect at (x: {}, y: {})",
                    x,
                    y
                );
                assert_eq!(
                    mask_13.get(x, y) == 1,
                    1 <= x && x <= WIDTH && 1 <= y && y <= HEIGHT + 1,
                    "mask_13 is incorrect at (x: {}, y: {})",
                    x,
                    y
                );
                assert!(
                    full_mask.get(x, y) == 1,
                    "full_mask is incorrect at (x: {}, y: {})",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn onebit() {
        assert_eq!(
            BoardBits::onebit(3, 2),
            BoardBits::from(concat!(
                "..1...", // 2
                "......", // 1
            )),
        )
    }

    #[test]
    fn bit_ops() {
        let lhs = BoardBits::from("0011..");
        let rhs = BoardBits::from("0101..");

        assert_eq!(lhs & rhs, BoardBits::from("0001.."));
        assert_eq!(lhs | rhs, BoardBits::from("0111.."));
        assert_eq!(lhs ^ rhs, BoardBits::from("0110.."));
        assert_eq!(lhs.andnot(rhs), BoardBits::from("0100.."))
    }

    #[test]
    fn shift_basic() {
        let bb = BoardBits::onebit(2, 3);

        assert_eq!(bb.shift_up(), BoardBits::onebit(2, 4));
        assert_eq!(bb.shift_down(), BoardBits::onebit(2, 2));
        assert_eq!(bb.shift_left(), BoardBits::onebit(1, 3));
        assert_eq!(bb.shift_right(), BoardBits::onebit(3, 3));
    }

    #[test]
    fn shift_corner() {
        assert!(BoardBits::onebit(3, 15).shift_up().is_zero());
        assert!(BoardBits::onebit(3, 0).shift_down().is_zero());
        assert!(BoardBits::onebit(0, 3).shift_left().is_zero());
        assert!(BoardBits::onebit(7, 3).shift_right().is_zero());
    }

    #[test]
    fn popcount() {
        assert_eq!(BoardBits::zero().popcount(), 0);
        assert_eq!(BoardBits::onebit(2, 3).popcount(), 1);
        assert_eq!(
            BoardBits::from(concat!(
                "11.1.1", // 2
                ".11.1.", // 1
            ))
            .popcount(),
            7,
        );
    }

    #[test]
    fn lsb() {
        // low
        assert_eq!(
            BoardBits::from(concat!(
                "11.1.1", // 2
                ".11.1.", // 1
            ))
            .lsb(),
            BoardBits::from(concat!(
                "1.....", // 2
                "......", // 1
            ))
        );
        // high
        assert_eq!(
            BoardBits::from(concat!(
                "...1.1", // 2
                "....1.", // 1
            ))
            .lsb(),
            BoardBits::from(concat!(
                "...1..", // 2
                "......", // 1
            ))
        );
    }

    #[test]
    fn max_u16x8() {
        let testcases = [
            (
                [
                    0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
                ],
                0x0000,
            ),
            (
                [
                    0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                ],
                0xFFFF,
            ),
            (
                [
                    0xABC7, 0xABC3, 0xABC4, 0xABC0, 0xABC6, 0xABC1, 0xABC2, 0xABC5,
                ],
                0xABC7,
            ),
            (
                [
                    0xABCD, 0x0000, 0x0001, 0xFFFF, 0xDEAD, 0xBEEF, 0x000F, 0xFFFE,
                ],
                0xFFFF,
            ),
        ];

        for (bits, max) in testcases {
            let bb: BoardBits = bits.into();
            assert_eq!(bb.max_u16x8(), max);
        }
    }

    #[test]
    fn popcount_u16x8() {
        let testcases = [
            (
                [
                    0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
                ],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ),
            (
                [
                    0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                ],
                [16, 16, 16, 16, 16, 16, 16, 16],
            ),
            (
                [
                    0xABC7, 0xABC3, 0xABC4, 0xABC0, 0xABC6, 0xABC1, 0xABC2, 0xABC5,
                ],
                [10, 9, 8, 7, 9, 8, 8, 9],
            ),
            (
                [
                    0xABCD, 0x0000, 0x0001, 0xFFFF, 0xDEAD, 0xBEEF, 0x000F, 0xFFFE,
                ],
                [10, 0, 1, 16, 11, 13, 4, 15],
            ),
        ];

        for (bits, pc) in testcases {
            let bb: BoardBits = bits.into();
            let expected: BoardBits = pc.into();
            assert_eq!(bb.popcount_u16x8(), expected);
        }
    }

    #[test]
    fn popcount_u16x8_array() {
        let testcases = [
            (
                [
                    0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
                ],
                [0, 0, 0, 0, 0, 0, 0, 0],
            ),
            (
                [
                    0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                ],
                [16, 16, 16, 16, 16, 16, 16, 16],
            ),
            (
                [
                    0xABC7, 0xABC3, 0xABC4, 0xABC0, 0xABC6, 0xABC1, 0xABC2, 0xABC5,
                ],
                [10, 9, 8, 7, 9, 8, 8, 9],
            ),
            (
                [
                    0xABCD, 0x0000, 0x0001, 0xFFFF, 0xDEAD, 0xBEEF, 0x000F, 0xFFFE,
                ],
                [10, 0, 1, 16, 11, 13, 4, 15],
            ),
        ];

        for (bits, expected) in testcases {
            let bb: BoardBits = bits.into();
            assert_eq!(bb.popcount_u16x8_array(), expected);
        }
    }

    #[test]
    fn set_below_top_one_u16x8() {
        let testcases = [
            (
                [
                    0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
                ],
                [
                    0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
                ],
            ),
            (
                [
                    0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                ],
                [
                    0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
                ],
            ),
            (
                [
                    0x00BB, 0x0000, 0x0001, 0x06F0, 0xDEAD, 0x01E2, 0x0503, 0xFFFE,
                ],
                [
                    0x00FF, 0x0000, 0x0001, 0x07FF, 0xFFFF, 0x01FF, 0x07FF, 0xFFFF,
                ],
            ),
        ];

        for (bits, spl) in testcases {
            let bb: BoardBits = bits.into();
            let expected: BoardBits = spl.into();
            assert_eq!(bb.set_below_top_one_u16x8(), expected);
        }
    }

    #[test]
    fn pext_u64() {
        assert_eq!(
            BoardBits::pext_u64(0b1010_1100_1010_1011, 0b0101_0110_0101_1111),
            0b0010001011
        );
        assert_eq!(
            BoardBits::pext_u64(0b0110_1001_1011_1111, 0b0111_1100_1100_0110),
            0b110101011
        );
        assert_eq!(BoardBits::pext_u64(0b0110_1001_1011_1111, 0), 0);
        assert_eq!(
            BoardBits::pext_u64(0b0110_1001_1011_1111, u64::MAX),
            0b0110_1001_1011_1111
        );
        assert_eq!(BoardBits::pext_u64(0, 0), 0);
        assert_eq!(BoardBits::pext_u64(0, u64::MAX), 0);
        assert_eq!(BoardBits::pext_u64(u64::MAX, 0), 0);
        assert_eq!(BoardBits::pext_u64(u64::MAX, u64::MAX), u64::MAX);
    }

    #[test]
    fn pdep_u64() {
        assert_eq!(
            BoardBits::pdep_u64(0b1010_1100_1010_1011, 0b0101_0110_0101_1111),
            0b0000_0100_0100_1011
        );
        assert_eq!(
            BoardBits::pdep_u64(0b0110_1001_1011_1111, 0b0111_1100_1100_0110),
            0b0110_1100_1100_0110
        );
        assert_eq!(BoardBits::pdep_u64(0b0110_1001_1011_1111, 0), 0);
        assert_eq!(
            BoardBits::pdep_u64(0b0110_1001_1011_1111, u64::MAX),
            0b0110_1001_1011_1111
        );
        assert_eq!(BoardBits::pdep_u64(0, 0), 0);
        assert_eq!(BoardBits::pdep_u64(0, u64::MAX), 0);
        assert_eq!(BoardBits::pdep_u64(u64::MAX, 0), 0);
        assert_eq!(BoardBits::pdep_u64(u64::MAX, u64::MAX), u64::MAX);
    }

    #[test]
    fn before_pop_mask() {
        assert_eq!(
            BoardBits::before_pop_mask(
                BoardBits::from(concat!(
                    "1..111", // 3
                    "11.1..", // 2
                    ".1.1..", // 1
                )) ^ BoardBits::wall()
            ),
            BoardBits::from(concat!(
                "111111", // 14
                "111111", // 13
                "111111", // 12
                "111111", // 11
                "111111", // 10
                "111111", // 9
                "111111", // 8
                "111111", // 7
                "111111", // 6
                "111111", // 5
                "111111", // 4
                ".11...", // 3
                "..1.11", // 2
                "1.1.11", // 1
            ))
            .into()
        );
    }

    #[test]
    fn after_pop_mask() {
        assert_eq!(
            BoardBits::after_pop_mask(BoardBits::from(concat!(
                "1..111", // 3
                "11.1..", // 2
                ".1.1..", // 1
            ))),
            ((BoardBits::from(concat!(
                "..1...", // 14
                "..1.11", // 13
                "111.11", // 12
                "111111", // 11
                "111111", // 10
                "111111", // 9
                "111111", // 8
                "111111", // 7
                "111111", // 6
                "111111", // 5
                "111111", // 4
                "111111", // 3
                "111111", // 2
                "111111", // 1
            )) ^ BoardBits::wall())
            .shift_up()
                | BoardBits::wall().shift_up().shift_down()) // bottom-most row
            .into(),
        );
    }

    #[test]
    fn get_set() {
        let mut bb = BoardBits::zero();
        assert_eq!(bb.get(2, 3), 0);

        bb.set_1(2, 3);
        assert_eq!(bb.get(2, 3), 1);

        bb.set_0(2, 3);
        assert_eq!(bb.get(2, 3), 0);
    }

    #[test]
    fn is_zero() {
        assert!(BoardBits::zero().is_zero());
        assert!(!BoardBits::onebit(2, 3).is_zero());
    }
}
