use std::mem;

use super::{
    boardbits::{BoardBits, BoardOps},
    HEIGHT,
};
use crate::{
    board::{ENTIRE_WIDTH, WIDTH},
    chain::{frame, score, Chain},
    color::PuyoColor,
};

#[derive(Clone, PartialEq, Debug)]
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

    pub fn height_array(&self) -> [usize; ENTIRE_WIDTH] {
        (self.0 | self.1 | self.2)
            .mask_13()
            .popcount_u16x8_array()
            // convert to [usize; _] for convenience
            .iter()
            .map(|&x| x as usize)
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap()
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

    pub fn escape_above_13th_row(&mut self) -> Board {
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

    pub fn unescape_above_13th_row(&mut self, escaped: &Board) {
        self.0 = self.0 | escaped.0;
        self.1 = self.1 | escaped.1;
        self.2 = self.2 | escaped.2;
    }

    /// Return (BoardBits of popping_puyos, num_popped_puyos, color_bonus, conn_bonus)
    pub fn popping_puyos(&self) -> Option<(BoardBits, u32, u32, u32)> {
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

        Some((
            popped_puyos,
            num_popped_puyos as u32,
            color_bonus,
            conn_bonus,
        ))
    }

    pub fn pop_and_apply_gravity(&mut self, popped_puyos: BoardBits) {
        let (bef_lo, bef_hi) = BoardBits::before_pop_mask(popped_puyos);
        let (aft_lo, aft_hi) = BoardBits::after_pop_mask(popped_puyos);

        let mut b: [(u64, u64); 3] = [self.0.into(), self.1.into(), self.2.into()];
        if aft_lo != 0xFFFFFFFFFFFFFFFF {
            b[0].0 = BoardBits::pdep_u64(BoardBits::pext_u64(b[0].0, bef_lo), aft_lo);
            b[1].0 = BoardBits::pdep_u64(BoardBits::pext_u64(b[1].0, bef_lo), aft_lo);
            b[2].0 = BoardBits::pdep_u64(BoardBits::pext_u64(b[2].0, bef_lo), aft_lo);
            if aft_hi != 0xFFFFFFFFFFFFFFFF {
                b[0].1 = BoardBits::pdep_u64(BoardBits::pext_u64(b[0].1, bef_hi), aft_hi);
                b[1].1 = BoardBits::pdep_u64(BoardBits::pext_u64(b[1].1, bef_hi), aft_hi);
                b[2].1 = BoardBits::pdep_u64(BoardBits::pext_u64(b[2].1, bef_hi), aft_hi);
            }
        } else {
            b[0].1 = BoardBits::pdep_u64(BoardBits::pext_u64(b[0].1, bef_hi), aft_hi);
            b[1].1 = BoardBits::pdep_u64(BoardBits::pext_u64(b[1].1, bef_hi), aft_hi);
            b[2].1 = BoardBits::pdep_u64(BoardBits::pext_u64(b[2].1, bef_hi), aft_hi);
        }

        self.0 = b[0].into();
        self.1 = b[1].into();
        self.2 = b[2].into();
    }

    pub fn max_drops(&self, popped_puyos: BoardBits) -> u16 {
        // it makes no sense to use this method for board with top wall.
        debug_assert!(popped_puyos.not_mask_13().is_zero());

        let dropping_puyos = popped_puyos.andnot(self.0 | self.1 | self.2);
        let holes = dropping_puyos.set_below_top_one_u16x8() & popped_puyos;
        holes.popcount_u16x8().max_u16x8() as u16
    }

    pub fn simulate(&mut self) -> Chain {
        let escaped = self.escape_above_13th_row();

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
            frame += frame::chain_frames(self.max_drops(popped_puyos));

            self.pop_and_apply_gravity(popped_puyos);
        }

        self.unescape_above_13th_row(&escaped);
        Chain::new(chain as u32, (score * 10) as u32, frame)
    }

    pub fn place_puyo(&mut self, x: usize, c: PuyoColor) {
        let y_ = self.height_array()[x];
        debug_assert!(y_ + 1 <= 14);

        self.set(x, y_ + 1, c)
    }

    pub fn is_dead(&self) -> bool {
        self.get(3, HEIGHT) != PuyoColor::EMPTY
    }
}

impl From<&'static str> for Board {
    fn from(value: &'static str) -> Self {
        debug_assert!(value.len() % WIDTH == 0);

        let mut board = Self::new();

        for (y_, chunk) in value.as_bytes().chunks(WIDTH).rev().enumerate() {
            for (x_, c) in chunk.iter().enumerate() {
                // x and y are both one-based
                board.set(x_ + 1, y_ + 1, PuyoColor::from(*c));
            }
        }

        board
    }
}

/// For tests.
impl From<[usize; WIDTH]> for Board {
    fn from(heights: [usize; WIDTH]) -> Self {
        let mut board = Board::new();
        for (x_, h) in heights.iter().enumerate() {
            let x = x_ + 1;
            for y in 1..=*h {
                board.set(x, y, PuyoColor::OJAMA)
            }
        }

        board
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{board::ENTIRE_HEIGHT, color::PuyoColor::*};

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
    fn height_array() {
        let board = Board::new();
        assert_eq!(board.height_array()[1..=6], [0, 0, 0, 0, 0, 0]);

        let board = Board::from(concat!(
            "O..O..", // 4
            "OO.O..", // 4
            "OO.O.O", // 4
            "OO.OOO", // 4
        ));
        assert_eq!(board.height_array()[1..=6], [4, 3, 0, 4, 1, 2]);
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

    #[test]
    fn escape_above_13th_row() {
        let mut full_board = Board(
            BoardBits::full_mask(),
            BoardBits::full_mask(),
            BoardBits::full_mask(),
        );

        let escaped = full_board.escape_above_13th_row();

        assert!((full_board.0 & escaped.0).is_zero());
        assert!((full_board.1 & escaped.1).is_zero());
        assert!((full_board.2 & escaped.2).is_zero());
        assert!(full_board.0.not_mask_13().is_zero());
        assert!(full_board.1.not_mask_13().is_zero());
        assert!(full_board.2.not_mask_13().is_zero());
        assert!(escaped.0.mask_13().is_zero());
        assert!(escaped.1.mask_13().is_zero());
        assert!(escaped.2.mask_13().is_zero());
    }

    #[test]
    fn unescape_above_13th_row() {
        let mut full_board = Board(
            BoardBits::full_mask(),
            BoardBits::full_mask(),
            BoardBits::full_mask(),
        );
        let escaped = full_board.escape_above_13th_row();

        full_board.unescape_above_13th_row(&escaped);

        assert_eq!(full_board.0, BoardBits::full_mask());
        assert_eq!(full_board.1, BoardBits::full_mask());
        assert_eq!(full_board.2, BoardBits::full_mask());
    }

    #[test]
    fn popping_puyos() {
        assert_eq!(
            Board::from(concat!(
                "B.....", // 6
                "RY....", // 5
                "RY....", // 4
                "RBBG..", // 3
                "YBGG..", // 2
                "RYBG.."  // 1
            ))
            .popping_puyos(),
            Some((
                BoardBits::from(concat!(
                    "...1..", // 3
                    "..11..", // 2
                    "...1..", // 1
                )),
                4,                     // num_popped_puyos
                score::color_bonus(1), // color_bonus
                score::conn_bonus(4),  // conn_bonus
            ))
        );

        assert_eq!(
            Board::from(concat!(
                "..RGG.", // 4
                ".BBYY.", // 3
                "RRBBYY", // 2
                "RRRGYY"  // 1
            ))
            .popping_puyos(),
            Some((
                BoardBits::from(concat!(
                    ".1111.", // 3
                    "111111", // 2
                    "111.11", // 1
                )),
                5 + 4 + 6,             // num_popped_puyos
                score::color_bonus(3), // color_bonus
                score::conn_bonus(5) + score::conn_bonus(4) + score::conn_bonus(6), // conn_bonus
            ))
        );
    }

    #[test]
    fn pop_and_apply_gravity() {
        let testcases = [
            (
                Board::from(concat!(
                    "B.....", // 6
                    "RY....", // 5
                    "RY....", // 4
                    "RBBG..", // 3
                    "YBGG..", // 2
                    "RYBG.."  // 1
                )),
                BoardBits::from(concat!(
                    "...1..", // 3
                    "..11..", // 2
                    "...1..", // 1
                )),
                Board::from(concat!(
                    "B.....", // 6
                    "RY....", // 5
                    "RY....", // 4
                    "RB....", // 3
                    "YBB...", // 2
                    "RYB..."  // 1
                )),
            ),
            (
                Board::from(concat!(
                    "..RGG.", // 4
                    ".BBYY.", // 3
                    "RRBBYY", // 2
                    "RRRGYY"  // 1
                )),
                BoardBits::from(concat!(
                    ".1111.", // 3
                    "111111", // 2
                    "111.11", // 1
                )),
                Board::from(concat!(
                    "...G..", // 2
                    "..RGG."  // 1
                )),
            ),
        ];

        for (mut before, popped, mut after) in testcases {
            before.escape_above_13th_row();
            after.escape_above_13th_row();

            before.pop_and_apply_gravity(popped);

            assert_eq!(before, after)
        }
    }

    #[test]
    fn max_drops() {
        let testcases = [
            (
                Board::from(concat!(
                    ".G....", // 5
                    ".RR...", // 4
                    ".GR...", // 3
                    ".RR...", // 2
                    ".RR...", // 1
                )),
                BoardBits::from(concat!(
                    ".11...", // 4
                    "..1...", // 3
                    ".11...", // 2
                    ".11...", // 1
                )),
                3,
            ),
            (
                Board::from(concat!(
                    "..RGG.", // 4
                    ".BBYY.", // 3
                    "RRBBYY", // 2
                    "RRRGYY"  // 1
                )),
                BoardBits::from(concat!(
                    ".1111.", // 3
                    "111111", // 2
                    "111.11", // 1
                )),
                3,
            ),
            (Board::from("RRRR.G"), BoardBits::from("1111.."), 0),
        ];

        for (mut before, popped, max) in testcases {
            before.escape_above_13th_row();
            assert_eq!(before.max_drops(popped), max);
        }
    }

    #[test]
    fn simulate() {
        let board_and_chain = [
            (
                Board::from(concat!(
                    "B.....", // 6
                    "RY....", // 5
                    "RY....", // 4
                    "RBBG..", // 3
                    "YBGG..", // 2
                    "RYBG.."  // 1
                )),
                Chain::new(
                    4,
                    40 * 1 + 40 * 8 + 40 * 16 + 40 * 32,
                    frame::chain_frames(1)
                        + frame::chain_frames(2)
                        + frame::chain_frames(1)
                        + frame::chain_frames(4),
                ),
            ),
            (
                Board::from(concat!(
                    "RY....", // 5
                    "RY....", // 4
                    "RBBG..", // 3
                    "YBGG..", // 2
                    "RYBG.."  // 1
                )),
                Chain::new(
                    4,
                    40 * 1 + 40 * 8 + 40 * 16 + 40 * 32,
                    frame::chain_frames(1)
                        + frame::chain_frames(2)
                        + frame::chain_frames(1)
                        + frame::chain_frames(0),
                ),
            ),
            (
                Board::from(concat!(
                    "..BY..", // 4
                    "..BR..", // 3
                    "..BR..", // 2
                    ".BRR.."  // 1
                )),
                Chain::new(
                    2,
                    40 * 1 + 40 * 8,
                    frame::chain_frames(3) + frame::chain_frames(0),
                ),
            ),
            (
                Board::from(concat!(
                    ".G.BRG", // 13
                    "GBRRYR", // 12
                    "RRYYBY", // 11
                    "RGYRBR", // 10
                    "YGYRBY", // 9
                    "YGBGYR", // 8
                    "GRBGYR", // 7
                    "BRBYBY", // 6
                    "RYYBYY", // 5
                    "BRBYBR", // 4
                    "BGBYRR", // 3
                    "YGBGBG", // 2
                    "RBGBGG"  // 1
                )),
                // Note: haven't checked if frame is correct or not
                Chain::new(19, 175080, 1551),
            ),
        ];

        for (mut board, chain) in board_and_chain {
            assert_eq!(board.simulate(), chain);
        }
    }

    #[test]
    fn place_puyo() {
        let mut board = Board::new();
        board.place_puyo(1, RED);
        assert_eq!(board, Board::from("R....."));

        let mut board = Board::new();
        board.place_puyo(3, BLUE);
        assert_eq!(board, Board::from("..B..."));

        let mut board_bef = Board::from(concat!(
            "B.R..R", // 2
            "YYG.RR", // 1
        ));
        let board_aft = Board::from(concat!(
            "BOR..R", // 2
            "YYG.RR", // 1
        ));
        board_bef.place_puyo(2, OJAMA);
        assert_eq!(board_bef, board_aft);
    }

    #[test]
    fn is_dead() {
        assert!(!Board::new().is_dead());
        assert!(!Board::from(concat!(
            "..O...", // 11
            "..O...", // 10
            "..O...", // 9
            "..O...", // 8
            "..O...", // 7
            "..O...", // 6
            "..O...", // 5
            "..O...", // 4
            "..O...", // 3
            "..O...", // 2
            "..O...", // 1
        ))
        .is_dead());
        assert!(Board::from(concat!(
            "..O...", // 12
            "..O...", // 11
            "..O...", // 10
            "..O...", // 9
            "..O...", // 8
            "..O...", // 7
            "..O...", // 6
            "..O...", // 5
            "..O...", // 4
            "..O...", // 3
            "..O...", // 2
            "..O...", // 1
        ))
        .is_dead());
    }
}
