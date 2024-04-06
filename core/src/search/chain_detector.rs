use super::ComplementedPuyo;
use crate::{
    board::{Board, WIDTH},
    chain::Chain,
    color::PuyoColor,
    placement::Placement,
};

impl Board {
    pub fn detect_potential_chain<F>(&self, max_cmpl: u8, mut callback: F)
    where
        F: FnMut(Self, ComplementedPuyo, Chain),
    {
        let heights = self.height_array();

        for x in 1..=WIDTH {
            if !self.is_placeable(&Placement::new(x, 0)) {
                continue;
            }

            for c in PuyoColor::normal_colors() {
                let mut b = self.clone();
                let mut touched = false;

                // NOTE: starting from 2 (or 3) may be slightly better?
                for cmpl in 1..=max_cmpl {
                    b.place_puyo(x, *c);

                    touched |= (cmpl == 1 && heights[x] > 0 && self.get(x, heights[x]) == *c)
                        | (x > 1
                            && heights[x - 1] >= heights[x] + cmpl as usize
                            && self.get(x - 1, heights[x] + cmpl as usize) == *c)
                        | (x < WIDTH
                            && heights[x + 1] >= heights[x] + cmpl as usize
                            && self.get(x + 1, heights[x] + cmpl as usize) == *c);

                    if !touched {
                        continue;
                    }

                    let chain = b.simulate();
                    if chain.chain() == 0 {
                        continue;
                    }

                    callback(b, ComplementedPuyo::default().add(x, cmpl, *c), chain);
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use PuyoColor::*;

    use super::*;

    #[test]
    fn detect_potential_chain_cmpl_1() {
        let b = Board::from(concat!(
            "..Y...", // 7
            ".RY...", // 6
            ".RY...", // 5
            ".YRR..", // 4
            ".BGY..", // 3
            ".GYBB.", // 2
            "GGYBY.", // 1
        ));
        let mut expected: Vec<(ComplementedPuyo, Chain)> = vec![
            (
                ComplementedPuyo::default().add(1, 1, GREEN),
                Chain::new(2, 360, 162),
            ),
            (
                ComplementedPuyo::default().add(2, 1, YELLOW),
                Chain::new(1, 40, 55),
            ),
            (
                ComplementedPuyo::default().add(3, 1, YELLOW),
                Chain::new(1, 40, 55),
            ),
            (
                ComplementedPuyo::default().add(4, 1, YELLOW),
                Chain::new(1, 40, 55),
            ),
            (
                ComplementedPuyo::default().add(5, 1, BLUE),
                Chain::new(3, 1880, 248),
            ),
        ];

        let mut detected: Vec<(ComplementedPuyo, Chain)> = vec![];
        let callback = |_b: Board, cp: ComplementedPuyo, chain: Chain| {
            if cp.sum() == 1 {
                detected.push((cp, chain));
            }
        };
        b.detect_potential_chain(1, callback);

        detected.sort();
        expected.sort();
        assert_eq!(detected.len(), expected.len());
        assert_eq!(detected, expected);
    }

    #[test]
    fn detect_potential_chain_cmpl_2() {
        let b = Board::from(concat!(
            "..Y...", // 7
            ".RY...", // 6
            ".RY...", // 5
            ".YRR..", // 4
            ".BGY..", // 3
            ".GYBB.", // 2
            "GGYBY.", // 1
        ));
        let mut expected: Vec<(ComplementedPuyo, Chain)> = vec![
            (
                ComplementedPuyo::default().add(2, 2, RED),
                Chain::new(1, 40, 55),
            ),
            (
                ComplementedPuyo::default().add(4, 2, RED),
                Chain::new(2, 360, 160),
            ),
            (
                ComplementedPuyo::default().add(5, 2, RED),
                Chain::new(2, 360, 160),
            ),
            (
                ComplementedPuyo::default().add(6, 2, BLUE),
                Chain::new(3, 1940, 248),
            ),
        ];

        let mut detected: Vec<(ComplementedPuyo, Chain)> = vec![];
        let callback = |_b: Board, cp: ComplementedPuyo, chain: Chain| {
            if cp.sum() == 2 {
                detected.push((cp, chain));
            }
        };
        b.detect_potential_chain(2, callback);

        detected.sort();
        expected.sort();
        assert_eq!(detected.len(), expected.len());
        assert_eq!(detected, expected);
    }

    #[test]
    fn detect_potential_chain_cmpl_3() {
        let b = Board::from(concat!(
            "..Y...", // 7
            ".RY...", // 6
            ".RY...", // 5
            ".YRR..", // 4
            ".BGY..", // 3
            ".GYBB.", // 2
            "GGYBY.", // 1
        ));
        let mut expected: Vec<(ComplementedPuyo, Chain)> = vec![
            (
                ComplementedPuyo::default().add(1, 3, BLUE),
                Chain::new(2, 360, 160),
            ),
            (
                ComplementedPuyo::default().add(1, 3, YELLOW),
                Chain::new(2, 360, 160),
            ),
            (
                ComplementedPuyo::default().add(5, 3, YELLOW),
                Chain::new(1, 40, 80),
            ),
            (
                ComplementedPuyo::default().add(6, 3, YELLOW),
                Chain::new(1, 40, 80),
            ),
        ];

        let mut detected: Vec<(ComplementedPuyo, Chain)> = vec![];
        let callback = |_b: Board, cp: ComplementedPuyo, chain: Chain| {
            if cp.sum() == 3 {
                detected.push((cp, chain));
            }
        };
        b.detect_potential_chain(3, callback);

        detected.sort();
        expected.sort();
        assert_eq!(detected.len(), expected.len());
        assert_eq!(detected, expected);
    }
}
