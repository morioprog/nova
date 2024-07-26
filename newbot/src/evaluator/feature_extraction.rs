use core::{
    board::{Board, BoardOps, HEIGHT as H, WIDTH as W},
    color::PuyoColor::*,
};

pub(super) trait BoardFeature {
    fn bump(&self, x: usize) -> i32;
    fn dent(&self, x: usize) -> i32;
    fn dead_cells(&self) -> i32;
    fn connectivity(&self) -> (i32, i32);
    fn non_u_shape(&self) -> (i32, i32);
}

impl BoardFeature for Board {
    fn bump(&self, x: usize) -> i32 {
        if x == 1 || x == W {
            return 0;
        }

        let heights = self.height_array();

        let h_l = if x == 1 { 14 } else { heights[x - 1] as i32 };
        let h_x = heights[x] as i32;
        let h_r = if x == W { 14 } else { heights[x + 1] as i32 };

        let d_l = (h_x - h_l).max(0);
        let d_r = (h_x - h_r).max(0);

        d_l.min(d_r)
    }

    fn dent(&self, x: usize) -> i32 {
        let heights = self.height_array();

        let h_l = if x == 1 { 14 } else { heights[x - 1] as i32 };
        let h_x = heights[x] as i32;
        let h_r = if x == W { 14 } else { heights[x + 1] as i32 };

        let d_l = (h_l - h_x).max(0);
        let d_r = (h_r - h_x).max(0);

        d_l.min(d_r)
    }

    fn dead_cells(&self) -> i32 {
        let heights = self.height_array();
        let mut cells = 0;

        if heights[2] >= H && heights[1] < H {
            cells += H - heights[1];
        }
        if heights[4] >= H && heights[5] < H {
            cells += H - heights[5];
        }
        if (heights[4] >= H || heights[5] >= H) && heights[6] < H {
            cells += H - heights[6];
        }

        cells as i32
    }

    fn connectivity(&self) -> (i32, i32) {
        let mut conn_2 = 0;
        let mut conn_3 = 0;

        for color in [RED, GREEN, BLUE, YELLOW] {
            let b = self.bits_with_color(color);

            let u = b & b.shift_up();
            let d = b & b.shift_down();
            let l = b & b.shift_left();
            let r = b & b.shift_right();

            let (ud_and, ud_or) = (u & d, u | d);
            let (lr_and, lr_or) = (l & r, l | r);

            conn_2 += u.popcount() as i32;
            conn_2 += l.popcount() as i32;

            let conn_3_board = ud_and | lr_and | (ud_or & lr_or);
            conn_3 += conn_3_board.popcount() as i32;
        }

        // conn_3 consists of two conn_2s
        conn_2 -= conn_3 * 2;

        (conn_2, conn_3)
    }

    fn non_u_shape(&self) -> (i32, i32) {
        let heights = self.height_array();
        let avg_height = (heights[1..=W].iter().sum::<usize>() / W) as i32;

        let (mut sum, mut sq_sum) = (0, 0);
        for x in 1..=W {
            let ideal_height = match x {
                1 | 6 => avg_height + 2,
                2 | 5 => avg_height,
                3 | 4 => avg_height.saturating_sub(2),
                _ => unreachable!(),
            };

            let diff = ideal_height.abs_diff(heights[x] as i32) as i32;
            sum += diff;
            sq_sum += diff * diff;
        }

        (sum, sq_sum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bump() {
        let board: Board = [0, 4, 2, 6, 3, 3].into();

        assert_eq!(board.bump(1), 0);
        assert_eq!(board.bump(2), 2);
        assert_eq!(board.bump(3), 0);
        assert_eq!(board.bump(4), 3);
        assert_eq!(board.bump(5), 0);
        assert_eq!(board.bump(6), 0);
    }

    #[test]
    fn dent() {
        let board: Board = [3, 10, 6, 3, 9, 8].into();

        assert_eq!(board.dent(1), 7);
        assert_eq!(board.dent(2), 0);
        assert_eq!(board.dent(3), 0);
        assert_eq!(board.dent(4), 3);
        assert_eq!(board.dent(5), 0);
        assert_eq!(board.dent(6), 1);
    }

    #[test]
    fn dead_cells() {
        assert_eq!(Board::from([5, 12, 0, 0, 0, 0]).dead_cells(), 7);
        assert_eq!(Board::from([0, 0, 0, 12, 3, 6]).dead_cells(), 15);
        assert_eq!(Board::from([0, 0, 0, 0, 12, 4]).dead_cells(), 8);
        assert_eq!(Board::from([2, 12, 0, 12, 6, 3]).dead_cells(), 25);
        assert_eq!(Board::from([1, 12, 0, 0, 12, 4]).dead_cells(), 19);
        assert_eq!(Board::from([9, 12, 0, 12, 12, 2]).dead_cells(), 13);
    }

    #[test]
    fn connectivity() {
        let board = Board::from(concat!(
            "YRRRGG", // 3
            "YGGGBB", // 2
            "YRRRGG", // 1
        ));
        assert_eq!(board.connectivity(), (3, 4));

        let board = Board::from(concat!(
            ".YYG.R", // 5
            ".YGGYY", // 4
            "YGBBGB", // 3
            "YRGGBB", // 2
            "YRYYYG", // 1
        ));
        assert_eq!(board.connectivity(), (4, 5));
    }

    // TODO: add test for non-u-shape
}
