use super::ComplementedPuyo;
use crate::{
    board::{Board, BoardBits, BoardOps, WIDTH},
    chain::Chain,
    color::{Color, PuyoColor},
    placement::Placement,
};

impl Board {
    pub fn detect_potential_chain<Callback>(
        &self,
        max_cmpl: u8,
        max_depth: u8,
        mut callback: Callback,
    ) where
        Callback: FnMut(Self, usize, ComplementedPuyo, Chain),
    {
        let initial_heights = self.height_array();
        // (Board, depth, ComplementedPuyo, fire x, banned x)
        let mut stack: Vec<(Board, u8, ComplementedPuyo, usize, u8)> =
            vec![(self.clone(), 1, ComplementedPuyo::default(), usize::MAX, 0)];

        while let Some((board, depth, cp, fire_x, mut banned)) = stack.pop() {
            let heights = board.height_array();
            let max_cmpl = if depth == 1 { max_cmpl } else { 2 };

            for x in 1..=WIDTH {
                if banned >> x & 1 == 1 {
                    continue;
                }
                // Not extendable if no height change.
                if depth > 1 && initial_heights[x] == heights[x] {
                    banned |= 1 << x;
                    continue;
                }
                if depth == 1 && !self.is_placeable(&Placement::new(x, 0)) {
                    banned |= 1 << x;
                    continue;
                }

                for c in PuyoColor::normal_colors() {
                    let mut new_board = board.clone();
                    let mut touched = false;

                    // NOTE: starting from 2 (or 3) may be slightly better?
                    for cmpl in 1..=max_cmpl {
                        // Cannot be complemented.
                        if initial_heights[x] + cp.get(x) as usize + cmpl as usize >= 14 {
                            break;
                        }
                        new_board.place_puyo(x, *c);

                        // The one below is the same color.
                        // (Check only if cmpl == 1 since this will be always true for cmpl > 1.)
                        touched |= cmpl == 1 && heights[x] > 0 && self.get(x, heights[x]) == *c;
                        // The one on the left is the same color.
                        touched |= x > 1
                            && heights[x - 1] >= heights[x] + cmpl as usize
                            && self.get(x - 1, heights[x] + cmpl as usize) == *c;
                        // The one on the right is the same color.
                        touched |= x < WIDTH
                            && heights[x + 1] >= heights[x] + cmpl as usize
                            && self.get(x + 1, heights[x] + cmpl as usize) == *c;

                        // No existing adjacent puyos are the same color as complemented ones.
                        if !touched {
                            continue;
                        }

                        // Nothing vanishes.
                        if new_board.adjacent_count(x, heights[x] + cmpl as usize) < 4 {
                            continue;
                        }

                        // Something vanishes when complementing to initial board.
                        if depth > 1 && !self.clone().valid_extension(x, *c, cmpl) {
                            break;
                        }

                        let chain = new_board.simulate();
                        if chain.chain() == 0 {
                            continue;
                        }

                        let fire_x = if depth == 1 { x } else { fire_x };
                        let new_cp = cp.clone().add(x, cmpl, *c);
                        callback(new_board.clone(), fire_x, new_cp.clone(), chain);
                        if depth < max_depth {
                            stack.push((
                                new_board.clone(),
                                depth + 1,
                                new_cp,
                                fire_x,
                                if depth == 1 {
                                    // Ban the initial fire x.
                                    banned | (1 << x)
                                } else {
                                    banned
                                },
                            ));
                        }

                        break;
                    }
                }
            }
        }
    }

    fn adjacent_count(&self, x: usize, y: usize) -> usize {
        debug_assert!(self.get(x, y).is_normal_color());

        BoardBits::onebit(x, y)
            .expand(self.bits_with_color(self.get(x, y)))
            .popcount()
    }

    fn valid_extension(mut self, x: usize, color: PuyoColor, cmpl: u8) -> bool {
        for _ in 0..cmpl {
            self.place_puyo(x, color);
        }

        self.adjacent_count(x, self.height_array()[x]) < 4
    }
}

#[cfg(test)]
mod tests {
    use PuyoColor::*;

    use super::*;

    // TODO: add tests for max_depth > 1

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
        let callback = |_b: Board, _x: usize, cp: ComplementedPuyo, chain: Chain| {
            if cp.sum() == 1 {
                detected.push((cp, chain));
            }
        };
        b.detect_potential_chain(1, 1, callback);

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
        let callback = |_b: Board, _x: usize, cp: ComplementedPuyo, chain: Chain| {
            if cp.sum() == 2 {
                detected.push((cp, chain));
            }
        };
        b.detect_potential_chain(2, 1, callback);

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
        let callback = |_b: Board, _x: usize, cp: ComplementedPuyo, chain: Chain| {
            if cp.sum() == 3 {
                detected.push((cp, chain));
            }
        };
        b.detect_potential_chain(3, 1, callback);

        detected.sort();
        expected.sort();
        assert_eq!(detected.len(), expected.len());
        assert_eq!(detected, expected);
    }
}
