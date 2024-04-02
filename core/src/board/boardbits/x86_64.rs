use std::{arch::x86_64::*, simd::*};

use super::BoardOps;

#[repr(align(16))]
#[derive(Clone, Copy)]
pub struct BoardBits(__m128i);

impl BoardOps for BoardBits {
    fn andnot(&self, rhs: Self) -> Self {
        Self(unsafe { _mm_andnot_si128(self.0, rhs.0) })
    }

    fn shift_up(&self) -> Self {
        Self(unsafe { _mm_slli_epi16(self.0, 1) })
    }

    fn shift_down(&self) -> Self {
        Self(unsafe { _mm_srli_epi16(self.0, 1) })
    }

    fn shift_left(&self) -> Self {
        Self(unsafe { _mm_srli_si128(self.0, 2) })
    }

    fn shift_right(&self) -> Self {
        Self(unsafe { _mm_slli_si128(self.0, 2) })
    }

    fn popcount(&self) -> usize {
        let lh: i64x2 = self.0.into();
        unsafe { (_popcnt64(lh[0]) + _popcnt64(lh[1])) as usize }
    }

    fn lsb(&self) -> Self {
        debug_assert!(!self.is_zero());

        let lh: u64x2 = self.0.into();
        if lh[0] > 0 {
            return (1 << lh[0].trailing_zeros(), 0).into();
        }

        (0, 1 << lh[1].trailing_zeros()).into()
    }

    fn max_u16x8(&self) -> u16 {
        let inv = *self ^ Self::full_mask();
        let min_inv = unsafe { _mm_minpos_epu16(inv.0) };

        unsafe { !(_mm_cvtsi128_si32(min_inv) as u16) }
    }

    #[cfg(any(target_feature = "avx512bitalg", target_feature = "avx512vl"))]
    fn popcount_u16x8(&self) -> Self {
        Self(unsafe { _mm_popcnt_epi16(self.0) })
    }

    #[cfg(not(any(target_feature = "avx512bitalg", target_feature = "avx512vl")))]
    fn popcount_u16x8(&self) -> Self {
        Self(unsafe {
            let mask4 = _mm_set1_epi8(0x0F);
            let lookup = _mm_setr_epi8(0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4);

            let low = _mm_and_si128(mask4, self.0);
            let high = _mm_and_si128(mask4, _mm_srli_epi16(self.0, 4));

            let low_count = _mm_shuffle_epi8(lookup, low);
            let high_count = _mm_shuffle_epi8(lookup, high);
            let count8 = _mm_add_epi8(low_count, high_count);

            let count16 = _mm_add_epi8(count8, _mm_slli_epi16(count8, 8));
            _mm_srli_epi16(count16, 8)
        })
    }

    fn set_below_top_one_u16x8(&self) -> Self {
        let mut b = self.0;
        b = unsafe { _mm_or_si128(b, _mm_srli_epi16(b, 1)) };
        b = unsafe { _mm_or_si128(b, _mm_srli_epi16(b, 2)) };
        b = unsafe { _mm_or_si128(b, _mm_srli_epi16(b, 4)) };
        b = unsafe { _mm_or_si128(b, _mm_srli_epi16(b, 8)) };
        Self(b)
    }

    fn pext_u64(a: u64, mask: u64) -> u64 {
        unsafe { _pext_u64(a, mask) }
    }

    fn pdep_u64(a: u64, mask: u64) -> u64 {
        unsafe { _pdep_u64(a, mask) }
    }

    #[cfg(any(target_feature = "avx512bw", target_feature = "avx512vl"))]
    fn after_pop_mask(popped: Self) -> (u64, u64) {
        Self(unsafe { _mm_srlv_epi16(Self::full_mask().0, popped.popcount_u16x8().0) }).into()
    }

    #[cfg(not(any(target_feature = "avx512bw", target_feature = "avx512vl")))]
    fn after_pop_mask(popped: Self) -> (u64, u64) {
        let lxhx: u64x4 = unsafe {
            let shift = _mm256_cvtepu16_epi32(popped.popcount_u16x8().0);
            let half_ones = _mm256_cvtepu16_epi32(Self::full_mask().0);
            let shifted = _mm256_srlv_epi32(half_ones, shift);
            _mm256_packus_epi32(shifted, shifted).into()
        };
        (lxhx[0], lxhx[2])
    }

    fn is_zero(&self) -> bool {
        unsafe { _mm_test_all_zeros(self.0, self.0) != 0 }
    }
}

impl PartialEq<Self> for BoardBits {
    fn eq(&self, other: &Self) -> bool {
        (*self ^ *other).is_zero()
    }
}

impl std::ops::BitAnd for BoardBits {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(unsafe { _mm_and_si128(self.0, rhs.0) })
    }
}

impl std::ops::BitOr for BoardBits {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(unsafe { _mm_or_si128(self.0, rhs.0) })
    }
}

impl std::ops::BitXor for BoardBits {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(unsafe { _mm_xor_si128(self.0, rhs.0) })
    }
}

impl From<(u64, u64)> for BoardBits {
    fn from(value: (u64, u64)) -> Self {
        Self(u64x2::from_array([value.0, value.1]).into())
    }
}

impl From<BoardBits> for (u64, u64) {
    fn from(value: BoardBits) -> Self {
        let lh: u64x2 = value.0.into();
        (lh[0], lh[1])
    }
}

impl From<[u16; 8]> for BoardBits {
    fn from(value: [u16; 8]) -> Self {
        Self(u16x8::from_array(value).into())
    }
}

impl From<BoardBits> for [u16; 8] {
    fn from(value: BoardBits) -> Self {
        let lh: u16x8 = value.0.into();
        lh.to_array()
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;

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
            let bb: BoardBits = unsafe { mem::transmute(u16x8::from_array(bits)) };
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
            let bb: BoardBits = unsafe { mem::transmute(u16x8::from_array(bits)) };
            let expected: BoardBits = unsafe { mem::transmute(u16x8::from_array(pc)) };
            assert_eq!(bb.popcount_u16x8(), expected);
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
            let bb: BoardBits = unsafe { mem::transmute(u16x8::from_array(bits)) };
            let expected: BoardBits = unsafe { mem::transmute(u16x8::from_array(spl)) };
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
    fn after_pop_mask() {
        assert_eq!(
            BoardBits::after_pop_mask(BoardBits::from(concat!(
                "1..111", // 3
                "11.1..", // 2
                ".1.1..", // 1
            ))),
            unsafe {
                mem::transmute(
                    (BoardBits::from(concat!(
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
                        | BoardBits::wall().shift_up().shift_down(), // bottom-most row
                )
            },
        );
    }

    #[test]
    fn is_zero() {
        assert!(BoardBits::zero().is_zero());
        assert!(!BoardBits::onebit(2, 3).is_zero());
    }
}
