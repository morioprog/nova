use std::mem;

use super::boardbits::{BoardBits, BoardManipulation};
use crate::{chain::Chain, color::PuyoColor};

pub struct Board(BoardBits, BoardBits, BoardBits);

impl Board {
    pub fn new() -> Self {
        // 0b001 = 1 = PuyoColor::WALL
        Self(BoardBits::wall(), BoardBits::zero(), BoardBits::zero())
    }

    pub fn get(&self, x: usize, y: usize) -> PuyoColor {
        let b0: u8 = self.0.get(x, y) as u8;
        let b1: u8 = (self.1.get(x, y) as u8) << 1;
        let b2: u8 = (self.2.get(x, y) as u8) << 2;

        unsafe { mem::transmute(b0 | b1 | b2) }
    }

    pub fn simulate(&self) -> Chain {
        todo!()

        /*
           1. iterate each colors
           2. if any puyo vanishes (mask 12), increment some params to calc score
           3. don't forget to remove ojama
           4. repeat till no change
        */
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{ENTIRE_HEIGHT, ENTIRE_WIDTH};

    #[test]
    fn new() {
        let board = Board::new();

        for x in 0..ENTIRE_WIDTH {
            for y in 0..ENTIRE_HEIGHT {
                assert_eq!(
                    board.get(x, y),
                    if x == 0 || x == ENTIRE_WIDTH - 1 || y == 0 || y == ENTIRE_HEIGHT - 1 {
                        PuyoColor::WALL
                    } else {
                        PuyoColor::EMPTY
                    },
                    "Board::new() is incorrect at (x: {}, y: {})",
                    x,
                    y
                );
            }
        }
    }
}
