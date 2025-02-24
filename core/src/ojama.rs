// ref. https://puyo-camp.jp/posts/71019

use rand::seq::SliceRandom;

use crate::{
    board::{Board, WIDTH},
    color::PuyoColor,
};

/// 各マスごとのお邪魔による硬直フレーム数
const OJAMA_FRAMES_BY_POSITION: &[&[u32; WIDTH + 1]; 15] = &[
    &[0, 0, 0, 0, 0, 0, 0],
    &[0, 60, 59, 64, 56, 62, 58],
    &[0, 58, 57, 62, 54, 60, 56],
    &[0, 56, 55, 60, 52, 58, 54],
    &[0, 54, 53, 57, 50, 56, 52],
    &[0, 52, 51, 55, 48, 53, 49],
    &[0, 50, 48, 53, 46, 51, 47],
    &[0, 47, 46, 50, 44, 49, 45],
    &[0, 45, 44, 47, 42, 46, 42],
    &[0, 42, 41, 44, 39, 43, 40],
    &[0, 39, 38, 41, 36, 40, 37],
    &[0, 36, 35, 38, 33, 37, 34],
    &[0, 32, 31, 34, 30, 33, 30],
    &[0, 28, 27, 29, 26, 28, 26],
    &[0, 22, 21, 23, 20, 22, 21], // 23 is blank in ref
];
/// "お邪魔数による硬直フレーム数"
const OJAMA_FRAMES_BY_QUANTITY: &[u32; 31] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 5, 6, 8, 9, 11, 12, 13, 14, 15, 16, 18, 19, 20, 21, 21, 22, 23,
    24, 24, 25, 25,
];

pub const fn ojama_frames_by_position(x: usize, y: usize) -> u32 {
    debug_assert!(1 <= x && x <= WIDTH);
    debug_assert!(1 <= y && y <= 14);
    OJAMA_FRAMES_BY_POSITION[y][x]
}

pub const fn ojama_frames_by_quantity(quan: usize) -> u32 {
    debug_assert!(1 <= quan && quan <= 30);
    OJAMA_FRAMES_BY_QUANTITY[quan as usize]
}

impl Board {
    /// drop ojama, and return frames till the next controllable state
    pub fn drop_ojama(&mut self, ojama: usize, cols_bit: Option<u8>) -> u32 {
        if ojama == 0 {
            return 0;
        }

        debug_assert!(ojama <= 30);
        let (rows, ones) = (ojama / WIDTH, ojama % WIDTH);
        let heights = self.height_array();

        // decide which columns to drop last few ojamas
        let cols: u8 = if let Some(c) = cols_bit {
            c
        } else {
            (0..WIDTH)
                .collect::<Vec<_>>()
                .choose_multiple(&mut rand::thread_rng(), ones)
                .fold(0, |m, &i| m | (1 << i))
        };
        debug_assert!(cols.count_ones() as usize == ones);

        let mut frames_by_position = 0;
        let frames_by_quantity = ojama_frames_by_quantity(ojama);

        for x in 1..=WIDTH {
            // how many ojamas will drop on col x
            let mut n = rows;
            if ((cols >> (x - 1)) & 1) == 1 {
                n += 1;
            }
            n = std::cmp::min(n, 14 - heights[x]);

            // the one on bottom is always the slowest one
            if n > 0 {
                frames_by_position = std::cmp::max(
                    frames_by_position,
                    ojama_frames_by_position(x, heights[x] + 1),
                );
            }
            for _ in 0..n {
                self.place_puyo(x, PuyoColor::OJAMA);
            }
        }

        frames_by_position + frames_by_quantity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drop_ojama_1() {
        let mut board = Board::new();

        let frame = board.drop_ojama(30, None);

        assert_eq!(
            board,
            Board::from(concat!(
                "oooooo", // 5
                "oooooo", // 4
                "oooooo", // 3
                "oooooo", // 2
                "oooooo", // 1
            ))
        );
        assert_eq!(frame, 64 + 25);
    }

    #[test]
    fn drop_ojama_2() {
        let mut board = Board::new();

        // [4, 4, 3, 3, 3, 3]
        let frame = board.drop_ojama(20, Some(0b000011));

        assert_eq!(
            board,
            Board::from(concat!(
                "oo....", // 4
                "oooooo", // 3
                "oooooo", // 2
                "oooooo", // 1
            ))
        );
        assert_eq!(frame, 64 + 18);
    }

    #[test]
    fn drop_ojama_3() {
        let mut board = Board::from(concat!(
            "r.br..", // 3
            "gbrgbr", // 2
            "brgbrg", // 1
        ));

        // [2, 1, 2, 1, 2, 2]
        let frame = board.drop_ojama(10, Some(0b110101));

        assert_eq!(
            board,
            Board::from(concat!(
                "o.o...", // 5
                "o.oooo", // 4
                "robroo", // 3
                "gbrgbr", // 2
                "brgbrg", // 1
            ))
        );
        assert_eq!(frame, 58 + 5);
    }
}
