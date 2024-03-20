cfg_if::cfg_if! {
    if #[cfg(all(target_arch = "x86_64"))] {
        mod x86_64;
        pub(crate) use self::x86_64::BoardBits;
    } else {
        todo!()
        // mod base;
        // pub(crate) use self::base::BoardBits;
    }
}

use crate::board::{ENTIRE_HEIGHT, ENTIRE_WIDTH};

pub(super) trait BoardOps:
    From<&'static str>
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
    // constructor
    // TODO: can we these const fn?
    fn zero() -> Self;
    fn wall() -> Self;
    fn board_mask_12() -> Self;
    fn board_mask_13() -> Self;
    fn full_mask() -> Self;
    fn onebit(x: usize, y: usize) -> Self;

    /// Calculate `(~a) & b`.
    fn andnot(&self, rhs: Self) -> Self;

    // getter / setter
    fn get(&self, x: usize, y: usize) -> u8;
    fn set_0(&mut self, x: usize, y: usize);
    fn set_1(&mut self, x: usize, y: usize);
}

impl BoardBits {
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

    /// Call set_0 if value == 0, otherwise call set_1
    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        if value == 0 {
            self.set_0(x, y)
        } else {
            self.set_1(x, y)
        }
    }
}

impl std::fmt::Display for BoardBits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in (0..ENTIRE_HEIGHT).rev() {
            for x in 0..ENTIRE_WIDTH {
                s.push(('0' as u8 + self.get(x, y)) as char);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{HEIGHT, WIDTH};

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
}
