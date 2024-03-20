use std::mem;

use super::boardbits::{BoardBits, BoardOps};
use crate::{board::WIDTH, chain::Chain, color::PuyoColor};

pub struct Board(BoardBits, BoardBits, BoardBits);

impl Board {
    pub fn new() -> Self {
        // 0b001 = 1 = PuyoColor::WALL
        Self(BoardBits::wall(), BoardBits::zero(), BoardBits::zero())
    }

    pub fn get(&self, x: usize, y: usize) -> PuyoColor {
        let b0 = self.0.get(x, y);
        let b1 = self.1.get(x, y) << 1;
        let b2 = self.2.get(x, y) << 2;

        unsafe { mem::transmute(b0 | b1 | b2) }
    }

    pub fn set(&mut self, x: usize, y: usize, c: PuyoColor) {
        let cb = c as u8;
        self.0.set(x, y, cb & 0b001);
        self.1.set(x, y, cb & 0b010);
        self.2.set(x, y, cb & 0b100);
    }

    pub fn bits_with_color(&self, c: PuyoColor) -> BoardBits {
        match c {
            PuyoColor::EMPTY => (self.0 | self.1 | self.2) ^ BoardBits::full_mask(),
            PuyoColor::WALL => self.1.andnot(self.2.andnot(self.0)),
            PuyoColor::OJAMA => self.0.andnot(self.2.andnot(self.1)),
            PuyoColor::IRON => self.2.andnot(self.0 & self.1),
            PuyoColor::RED => self.0.andnot(self.1.andnot(self.2)),
            PuyoColor::GREEN => self.1.andnot(self.0 & self.2),
            PuyoColor::BLUE => self.0.andnot(self.1 & self.2),
            PuyoColor::YELLOW => self.0 & self.1 & self.2,
        }
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

impl From<&'static str> for Board {
    fn from(value: &'static str) -> Self {
        debug_assert!(value.len() % WIDTH == 0);

        let mut board = Self::new();

        for (_y, chunk) in value.as_bytes().chunks(WIDTH).rev().enumerate() {
            for (_x, c) in chunk.iter().enumerate() {
                // x and y are both one-based
                board.set(_x + 1, _y + 1, PuyoColor::from(*c));
            }
        }

        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        board::{ENTIRE_HEIGHT, ENTIRE_WIDTH},
        color::PuyoColor::*,
    };

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

    #[test]
    fn from_str() {
        let board = Board::from(concat!(
            ".Orgby", // 2
            "RGBY#&", // 1
        ));

        assert_eq!(board.get(1, 1), RED);
        assert_eq!(board.get(2, 1), GREEN);
        assert_eq!(board.get(3, 1), BLUE);
        assert_eq!(board.get(4, 1), YELLOW);
        assert_eq!(board.get(5, 1), WALL);
        assert_eq!(board.get(6, 1), IRON);
        assert_eq!(board.get(1, 2), EMPTY);
        assert_eq!(board.get(2, 2), OJAMA);
        assert_eq!(board.get(3, 2), RED);
        assert_eq!(board.get(4, 2), GREEN);
        assert_eq!(board.get(5, 2), BLUE);
        assert_eq!(board.get(6, 2), YELLOW);
    }

    #[test]
    fn get_set() {
        let mut board = Board::new();
        assert_eq!(board.get(3, 4), EMPTY);

        board.set(3, 4, RED);
        assert_eq!(board.get(3, 4), RED);

        board.set(3, 4, YELLOW);
        assert_eq!(board.get(3, 4), YELLOW);
    }

    #[test]
    fn bits_with_color() {
        let board = Board::from(concat!(
            ".O#.YY", // 2
            "RG&YRB", // 1
        ));

        assert_eq!(
            board.bits_with_color(EMPTY),
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
                "111111", // 3
                "1..1..", // 2
                "......", // 1
            ))
        );
        assert_eq!(
            board.bits_with_color(WALL),
            BoardBits::from(concat!(
                "..1...", // 2
                "......", // 1
            )) | BoardBits::wall()
        );
        assert_eq!(
            board.bits_with_color(OJAMA),
            BoardBits::from(concat!(
                ".1....", // 2
                "......", // 1
            ))
        );
        assert_eq!(board.bits_with_color(IRON), BoardBits::from("..1..."));
        assert_eq!(board.bits_with_color(RED), BoardBits::from("1...1."));
        assert_eq!(board.bits_with_color(GREEN), BoardBits::from(".1...."));
        assert_eq!(board.bits_with_color(BLUE), BoardBits::from(".....1"));
        assert_eq!(
            board.bits_with_color(YELLOW),
            BoardBits::from(concat!(
                "....11", // 2
                "...1..", // 1
            ))
        );
    }
}
