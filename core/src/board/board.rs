use std::mem;

use super::boardbits::{BoardBits, BoardOps};
use crate::{
    board::WIDTH,
    chain::{score, Chain},
    color::PuyoColor,
};

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

    pub fn escape_14th_row(&mut self) -> Board {
        let escaped = Board(
            self.0.not_mask_13(),
            self.1.not_mask_13(),
            self.2.not_mask_13(),
        );

        self.0 = self.0.mask_13();
        self.1 = self.1.mask_13();
        self.2 = self.2.mask_13();

        escaped
    }

    pub fn unescape_14th_row(&mut self, escaped: &Board) {
        self.0 = self.0 | escaped.0;
        self.1 = self.1 | escaped.1;
        self.2 = self.2 | escaped.2;
    }

    /// Return (BoardBits of popping_puyos, num_popped_puyos, color_bonus, conn_bonus)
    pub fn popping_puyos(&self) -> Option<(BoardBits, usize, usize, usize)> {
        // 0b10x = RED (0b100) or GREEN (0b101)
        let b12_01 = self.1.andnot(self.2).mask_12();
        // 0b11x = BLUE (0b110) or YELLOW (0b111)
        let b12_11 = (self.1 & self.2).mask_12();

        let r = self.0.andnot(b12_01);
        let g = self.0 & b12_01;
        let b = self.0.andnot(b12_11);
        let y = self.0 & b12_11;

        let mut popped_puyos = BoardBits::zero();
        let mut num_popped_puyos = 0;
        let mut num_colors = 0;
        let mut conn_bonus = 0;

        for bb in &[r, g, b, y] {
            let Some(mut pop) = bb.popping_bits() else {
                continue;
            };

            popped_puyos = popped_puyos | pop;
            let popcount = pop.popcount();
            num_popped_puyos += popcount;
            num_colors += 1;

            // must be one connected component
            if pop.popcount() < 8 {
                conn_bonus += score::conn_bonus(popcount);
            } else {
                while !pop.is_zero() {
                    let conn = pop.lsb().expand(pop);
                    debug_assert!(conn.popcount() >= 4);
                    conn_bonus += score::conn_bonus(conn.popcount());
                    pop = pop ^ conn;
                }
            }
        }

        if num_popped_puyos == 0 {
            return None;
        }

        let color_bonus = score::color_bonus(num_colors);

        // remove ojama adjacent to `popped_puyos`
        popped_puyos = popped_puyos
            | popped_puyos
                .expand_1(self.bits_with_color(PuyoColor::OJAMA))
                .mask_12();

        Some((popped_puyos, num_popped_puyos, color_bonus, conn_bonus))
    }

    pub fn simulate(&mut self) -> Chain {
        let escaped = self.escape_14th_row();

        let mut chain = 0;
        let mut score = 0;
        let mut frame = 0;

        loop {
            let Some((popped_puyos, num_popped_puyos, color_bonus, conn_bonus)) =
                self.popping_puyos()
            else {
                break;
            };

            chain += 1;
            let chain_bonus = score::chain_bonus(chain);
            score += num_popped_puyos * (chain_bonus + color_bonus + conn_bonus).clamp(1, 999);
            // TODO: calc frame

            let dropping_puyos = popped_puyos.andnot(self.0 | self.1 | self.2);
            // quick: max_drops = 0 (unrelated ones) or 15 (all clear)
            let max_drops = dropping_puyos.lsb_u16x8().max_u16x8().trailing_zeros() - 1;

            // TODO: apply gravity
            // TODO: calc frame
        }

        self.unescape_14th_row(&escaped);
        Chain::new(chain as u32, (score * 10) as u32, frame)
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

    // TODO: add test for escape_14th_row
    // TODO: add test for unescape_14th_row
    // TODO: add test for popping_puyos
    // TODO: add test for simulate
}
