use super::color_counter::ColorCounter;
use crate::{board::ENTIRE_WIDTH, color::PuyoColor};

#[derive(Clone, Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ComplementedPuyo {
    cols: [ColorCounter; ENTIRE_WIDTH],
    sum: u8,
}

impl ComplementedPuyo {
    pub fn sum(&self) -> u8 {
        self.sum
    }

    pub fn add(&mut self, x: usize, cmpl: u8, color: PuyoColor) -> Self {
        self.cols[x].add_color(color, cmpl as u16);
        self.sum += cmpl;
        self.clone()
    }

    pub fn get(&self, x: usize) -> u16 {
        self.cols[x].sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum() {
        let mut cp = ComplementedPuyo::default();
        cp.add(1, 2, PuyoColor::RED);
        assert_eq!(cp.sum(), 2);

        cp.add(2, 3, PuyoColor::BLUE);
        assert_eq!(cp.sum(), 5);

        cp.add(1, 1, PuyoColor::YELLOW);
        assert_eq!(cp.sum(), 6);
    }

    #[test]
    fn add() {
        let mut cp = ComplementedPuyo::default();
        cp.add(1, 2, PuyoColor::RED);
        assert_eq!(cp.cols[1].get(PuyoColor::RED), 2);

        cp.add(2, 3, PuyoColor::BLUE);
        assert_eq!(cp.cols[2].get(PuyoColor::BLUE), 3);

        cp.add(1, 1, PuyoColor::YELLOW);
        assert_eq!(cp.cols[1].get(PuyoColor::YELLOW), 1);

        cp.add(1, 3, PuyoColor::RED);
        assert_eq!(cp.cols[1].get(PuyoColor::RED), 5);
    }
}
