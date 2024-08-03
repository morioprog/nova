use crate::color::{Color, PuyoColor};

/// [u4; 4]. Can store super small integer per each color.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColorCounter(u16);

impl ColorCounter {
    pub fn add_color(&mut self, c: PuyoColor, val: u16) {
        debug_assert!(c.is_normal_color());
        debug_assert!(val < 16);

        self.0 += val << Self::color_to_shift(c);
    }

    #[allow(dead_code)]
    pub fn get(&self, c: PuyoColor) -> u16 {
        (self.0 >> Self::color_to_shift(c)) & 0b1111
    }

    pub fn popcount(&self) -> u32 {
        self.0.count_ones()
    }

    fn color_to_shift(c: PuyoColor) -> u8 {
        debug_assert!(c.is_normal_color());

        (c as u8 ^ 4) << 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_get() {
        let mut cc = ColorCounter::default();
        cc.add_color(PuyoColor::RED, 2);
        assert_eq!(cc.get(PuyoColor::RED), 2);

        cc.add_color(PuyoColor::BLUE, 3);
        assert_eq!(cc.get(PuyoColor::BLUE), 3);

        cc.add_color(PuyoColor::RED, 1);
        assert_eq!(cc.get(PuyoColor::RED), 3);

        cc.add_color(PuyoColor::YELLOW, 15);
        assert_eq!(cc.get(PuyoColor::YELLOW), 15);
    }
}
