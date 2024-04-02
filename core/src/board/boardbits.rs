use super::{ENTIRE_HEIGHT, ENTIRE_WIDTH, WIDTH};

cfg_if::cfg_if! {
    if #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))] {
        mod x86_64;
        pub(crate) use self::x86_64::BoardBits;
    } else {
        mod base;
        pub(crate) use self::base::BoardBits;
    }
}

pub(super) trait BoardOps:
    From<&'static str>
    + From<(u64, u64)>
    + Into<(u64, u64)>
    + From<[u16; 8]>
    + Into<[u16; 8]>
    + Sized
    + Clone
    + Copy
    + PartialEq<Self>
    + std::fmt::Display
    + std::fmt::Debug
    + std::ops::BitAnd<Output = Self>
    + std::ops::BitOr<Output = Self>
    + std::ops::BitXor<Output = Self>
{
    /// Calculate `(~a) & b`.
    fn andnot(&self, rhs: Self) -> Self;
    fn shift_up(&self) -> Self;
    fn shift_down(&self) -> Self;
    fn shift_left(&self) -> Self;
    fn shift_right(&self) -> Self;
    fn popcount(&self) -> usize;
    fn lsb(&self) -> Self;
    fn max_u16x8(&self) -> u16;
    fn popcount_u16x8(&self) -> Self;
    /// Set 1 for all 0s under topmost 1 in each column.
    /// e.g. up 00010100 down
    ///      => 00011111
    fn set_below_top_one_u16x8(&self) -> Self;

    // pext / pdep
    fn pext_u64(a: u64, mask: u64) -> u64;
    fn pdep_u64(a: u64, mask: u64) -> u64;
    fn after_pop_mask(popped: Self) -> (u64, u64);

    fn is_zero(&self) -> bool;
}

impl BoardBits {
    pub fn zero() -> Self {
        (0, 0).into()
    }

    pub fn wall() -> Self {
        [
            0xFFFF, 0x8001, 0x8001, 0x8001, 0x8001, 0x8001, 0x8001, 0xFFFF,
        ]
        .into()
    }

    pub fn board_mask_12() -> Self {
        [
            0x0000, 0x1FFE, 0x1FFE, 0x1FFE, 0x1FFE, 0x1FFE, 0x1FFE, 0x0000,
        ]
        .into()
    }

    pub fn board_mask_13() -> Self {
        [
            0x0000, 0x3FFE, 0x3FFE, 0x3FFE, 0x3FFE, 0x3FFE, 0x3FFE, 0x0000,
        ]
        .into()
    }

    pub fn full_mask() -> Self {
        [
            0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF, 0xFFFF,
        ]
        .into()
    }

    pub fn onebit(x: usize, y: usize) -> Self {
        debug_assert!(Self::within_bound(x, y));

        let shift = ((x << 4) | y) & 0x3F; // x << 4: choose column by multiplying 16
        let hi = (x as u64) >> 2; // 1 if x >= 4 else 0
        let lo = hi ^ 1;

        (lo << shift, hi << shift).into()
    }

    pub fn mask(&self, mask: Self) -> Self {
        *self & mask
    }
    pub fn mask_12(&self) -> Self {
        self.mask(Self::board_mask_12())
    }
    pub fn mask_13(&self) -> Self {
        self.mask(Self::board_mask_13())
    }

    pub fn not_mask(&self, mask: Self) -> Self {
        mask.andnot(*self)
    }
    pub fn not_mask_12(&self) -> Self {
        self.not_mask(Self::board_mask_12())
    }
    pub fn not_mask_13(&self) -> Self {
        self.not_mask(Self::board_mask_13())
    }

    pub fn get(&self, x: usize, y: usize) -> u8 {
        debug_assert!(Self::within_bound(x, y));

        !(Self::onebit(x, y) & *self).is_zero() as u8
    }

    /// Call set_0 if value == 0, otherwise call set_1
    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        if value == 0 {
            self.set_0(x, y)
        } else {
            self.set_1(x, y)
        }
    }

    fn set_0(&mut self, x: usize, y: usize) {
        debug_assert!(Self::within_bound(x, y));

        *self = Self::onebit(x, y).andnot(*self)
    }

    fn set_1(&mut self, x: usize, y: usize) {
        debug_assert!(Self::within_bound(x, y));

        *self = *self | Self::onebit(x, y)
    }

    pub fn before_pop_mask(popped: Self) -> (u64, u64) {
        (popped ^ Self::full_mask()).into()
    }

    pub fn expand(&self, mask: Self) -> Self {
        let mut bb = *self;
        loop {
            let next_bb = bb.expand_1(mask);
            if bb == next_bb {
                return bb;
            }
            bb = next_bb;
        }
    }

    pub fn expand_1(&self, mask: Self) -> Self {
        mask & (*self
            | (self.shift_up() | self.shift_down())
            | (self.shift_left() | self.shift_right()))
    }

    pub fn popping_bits(&self) -> Option<Self> {
        let u = *self & self.shift_up();
        let d = *self & self.shift_down();
        let l = *self & self.shift_left();
        let r = *self & self.shift_right();

        let (ud_and, ud_or) = (u & d, u | d);
        let (lr_and, lr_or) = (l & r, l | r);

        let threes = (ud_and & lr_or) | (lr_and & ud_or);
        let twos = ud_and | lr_and | (ud_or & lr_or);

        let two_u = twos & twos.shift_up();
        let two_l = twos & twos.shift_left();

        let pb = threes | two_u | two_l;
        if pb.is_zero() {
            return None;
        }

        let two_d = twos & twos.shift_down();
        let two_r = twos & twos.shift_right();

        Some((pb | two_d | two_r).expand_1(*self))
    }

    const fn within_bound(x: usize, y: usize) -> bool {
        x < ENTIRE_WIDTH && y < ENTIRE_HEIGHT
    }
}

impl std::fmt::Display for BoardBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in (0..ENTIRE_HEIGHT).rev() {
            for x in 0..ENTIRE_WIDTH {
                s.push((b'0' + self.get(x, y)) as char);
            }
            s.push('\n')
        }

        write!(f, "{}", s)
    }
}

impl std::fmt::Debug for BoardBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n{}", self)
    }
}

impl From<&'static str> for BoardBits
where
    Self: BoardOps,
{
    fn from(value: &'static str) -> Self {
        debug_assert!(value.len() % WIDTH == 0);

        let mut bb = Self::zero();

        for (y_, chunk) in value.as_bytes().chunks(WIDTH).rev().enumerate() {
            for (x_, c) in chunk.iter().enumerate() {
                if c == &b'1' {
                    // x and y are both one-based
                    bb.set_1(x_ + 1, y_ + 1);
                }
            }
        }

        bb
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::HEIGHT;

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
                    y
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
    fn mask() {
        let mask_full = BoardBits::full_mask();
        let mask_12 = mask_full.mask_12();
        let mask_13 = mask_full.mask_13();

        for x in 0..ENTIRE_WIDTH {
            for y in 0..ENTIRE_HEIGHT {
                assert!(
                    mask_full.get(x, y) == 1,
                    "mask_full is incorrect at (x: {}, y: {})",
                    x,
                    y
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
            }
        }
    }

    #[test]
    fn not_mask() {
        let mask_full = BoardBits::full_mask();
        let not_mask_12 = mask_full.not_mask_12();
        let not_mask_13 = mask_full.not_mask_13();

        for x in 0..ENTIRE_WIDTH {
            for y in 0..ENTIRE_HEIGHT {
                assert!(
                    mask_full.get(x, y) == 1,
                    "mask_full is incorrect at (x: {}, y: {})",
                    x,
                    y
                );
                assert_eq!(
                    not_mask_12.get(x, y) == 1,
                    x < 1 || x > WIDTH || y < 1 || y > HEIGHT,
                    "not_mask_12 is incorrect at (x: {}, y: {})",
                    x,
                    y
                );
                assert_eq!(
                    not_mask_13.get(x, y) == 1,
                    x < 1 || x > WIDTH || y < 1 || y > HEIGHT + 1,
                    "not_mask_13 is incorrect at (x: {}, y: {})",
                    x,
                    y
                );
            }
        }
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
    fn expand_1() {
        assert_eq!(
            BoardBits::onebit(3, 2).expand_1(BoardBits::full_mask()),
            BoardBits::from(concat!(
                "..1...", // 3
                ".111..", // 2
                "..1...", // 1
            ))
        );
        assert_eq!(
            BoardBits::onebit(3, 2).expand_1(BoardBits::from(concat!(
                "111111", // 2
                "111111", // 1
            ))),
            BoardBits::from(concat!(
                ".111..", // 2
                "..1...", // 1
            ))
        );
    }

    #[test]
    fn popping_bits() {
        let bb = BoardBits::from(concat!(
            "1...11", // 6
            "1.1.11", // 5
            "1.1.1.", // 4
            "1.11.1", // 3
            ".1..11", // 2
            "111.1.", // 1
        ));
        assert_eq!(bb.popping_bits(), Some(bb));

        assert!(BoardBits::from(concat!(
            "11.111", // 5
            "..1...", // 4
            "11.1..", // 3
            ".1.1.1", // 2
            "1.1.11", // 1
        ))
        .popping_bits()
        .is_none());
    }

    #[test]
    fn from_str() {
        let bb = BoardBits::from(concat!(
            "11..1.", // 2
            ".1...1", // 1
        ));

        assert_eq!(bb.get(1, 1), 0);
        assert_eq!(bb.get(2, 1), 1);
        assert_eq!(bb.get(3, 1), 0);
        assert_eq!(bb.get(4, 1), 0);
        assert_eq!(bb.get(5, 1), 0);
        assert_eq!(bb.get(6, 1), 1);
        assert_eq!(bb.get(1, 2), 1);
        assert_eq!(bb.get(2, 2), 1);
        assert_eq!(bb.get(3, 2), 0);
        assert_eq!(bb.get(4, 2), 0);
        assert_eq!(bb.get(5, 2), 1);
        assert_eq!(bb.get(6, 2), 0);
    }
}
