use super::Placement;
use crate::{
    board::{Board, HEIGHT, WIDTH},
    tumo::Tumo,
};

impl Board {
    pub fn place_tumo(&mut self, tumo: &Tumo, placement: &Placement) -> Option<u32> {
        let frame = self.place_frames(placement)?;

        let (axis, child) = if placement.rot() == 2 {
            (tumo.child(), tumo.axis())
        } else {
            (tumo.axis(), tumo.child())
        };

        self.place_puyo(placement.axis_x(), axis);
        self.place_puyo(placement.child_x(), child);

        Some(frame)
    }

    pub fn valid_placements(&self, is_zoro: bool) -> Vec<&Placement> {
        let iter = if is_zoro {
            Placement::placements_zoro().iter()
        } else {
            Placement::placements_non_zoro().iter()
        };

        iter.filter(|pl| self.is_placeable(pl)).collect()
    }

    pub fn is_placeable(&self, placement: &Placement) -> bool {
        self.place_frames(placement).is_some()
    }

    // TODO: frame estimation (chigiri, vertical/horizontal move) & test
    pub fn place_frames(&self, placement: &Placement) -> Option<u32> {
        debug_assert!(placement.is_valid());

        let heights = self.height_array();
        let (x, r) = (placement.axis_x(), placement.rot());

        // child puyo cannot be placed on 14th row
        if r == 2 && heights[x] >= HEIGHT {
            return None;
        }

        // <=> (axis_x, child_x).max_by { abs(3 - x) }
        let x = if r == 1 && x >= 3 {
            x + 1
        } else if r == 3 && x <= 3 {
            x - 1
        } else {
            x
        };

        // either (3, 0) or (3, 2)
        if x == 3 {
            return Some(0);
        }

        let x_rng = if x < 3 { x..=2 } else { 4..=x };
        if x_rng.clone().any(|x| heights[x] > HEIGHT) {
            return None;
        }
        if x_rng.clone().all(|x| heights[x] < HEIGHT) {
            return Some(0);
        }

        if x < 3 {
            if heights[2] == HEIGHT && heights[4] >= HEIGHT {
                return Some(0);
            }

            let mut max_frames = 0;

            for i in x_rng.rev() {
                if heights[i] != HEIGHT {
                    continue;
                }

                let mut frames = None;

                for j in (i + 1)..=WIDTH {
                    if heights[j] == HEIGHT - 1 {
                        frames = Some(0);
                        break;
                    }
                    if heights[j] > HEIGHT {
                        return None;
                    }
                    if heights[j] == HEIGHT && j > 3 {
                        return None;
                    }
                }

                if frames.is_none() {
                    return None;
                }

                max_frames = max_frames.max(frames.unwrap());
            }

            return Some(max_frames);
        }

        // x > 3 below here

        if heights[4] == HEIGHT && heights[2] >= HEIGHT {
            return Some(0);
        }

        let mut max_frames = 0;

        for i in x_rng {
            if heights[i] != HEIGHT {
                continue;
            }

            let mut frames = None;

            for j in (1..=(i - 1)).rev() {
                if heights[j] == HEIGHT - 1 {
                    frames = Some(0);
                    break;
                }
                if heights[j] > HEIGHT {
                    return None;
                }
                if heights[j] == HEIGHT && j < 3 {
                    return None;
                }
            }

            if frames.is_none() {
                return None;
            }

            max_frames = max_frames.max(frames.unwrap());
        }

        Some(max_frames)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: write test for place_tumo
    // TODO: write test for place_frames

    #[test]
    fn is_placeable_empty_board() {
        let board: Board = [0, 0, 0, 0, 0, 0].into();

        for placement in Placement::placements_non_zoro() {
            let (x, r) = (placement.axis_x(), placement.rot());
            assert!(board.is_placeable(placement), "Failed at ({}, {})", x, r);
        }
    }

    #[test]
    fn is_placeable_upper_1() {
        let board: Board = [11, 11, 11, 11, 11, 11].into();

        for placement in Placement::placements_non_zoro() {
            let (x, r) = (placement.axis_x(), placement.rot());
            assert!(board.is_placeable(placement), "Failed at ({}, {})", x, r);
        }
    }

    #[test]
    fn is_placeable_upper_2() {
        let board: Board = [12, 12, 11, 12, 12, 12].into();
        let unreachables = [
            Placement::new(1, 2),
            Placement::new(2, 2),
            Placement::new(4, 2),
            Placement::new(5, 2),
            Placement::new(6, 2),
        ];

        for placement in Placement::placements_non_zoro() {
            assert_eq!(
                board.is_placeable(placement),
                !unreachables.contains(placement),
                "Failed at ({}, {})",
                placement.axis_x(),
                placement.rot()
            );
        }
    }

    #[test]
    fn is_placeable_upper_3() {
        let board: Board = [11, 11, 11, 13, 11, 11].into();
        let reachables = [
            Placement::new(1, 0),
            Placement::new(2, 0),
            Placement::new(3, 0),
            Placement::new(1, 1),
            Placement::new(2, 1),
            Placement::new(1, 2),
            Placement::new(2, 2),
            Placement::new(3, 2),
            Placement::new(2, 3),
            Placement::new(3, 3),
        ];

        for placement in Placement::placements_non_zoro() {
            assert_eq!(
                board.is_placeable(placement),
                reachables.contains(placement),
                "Failed at ({}, {})",
                placement.axis_x(),
                placement.rot()
            );
        }
    }

    #[test]
    fn is_placeable_upper_4() {
        let board: Board = [11, 13, 11, 11, 11, 11].into();
        let unreachables = [
            Placement::new(1, 0),
            Placement::new(2, 0),
            Placement::new(1, 1),
            Placement::new(2, 1),
            Placement::new(1, 2),
            Placement::new(2, 2),
            Placement::new(2, 3),
            Placement::new(3, 3),
        ];

        for placement in Placement::placements_non_zoro() {
            assert_eq!(
                board.is_placeable(placement),
                !unreachables.contains(placement),
                "Failed at ({}, {})",
                placement.axis_x(),
                placement.rot()
            );
        }
    }

    #[test]
    fn is_placeable_upper_5() {
        let board: Board = [11, 12, 11, 13, 11, 11].into();
        let reachables = [
            Placement::new(1, 0),
            Placement::new(2, 0),
            Placement::new(3, 0),
            Placement::new(1, 1),
            Placement::new(2, 1),
            Placement::new(1, 2),
            Placement::new(3, 2),
            Placement::new(2, 3),
            Placement::new(3, 3),
        ];

        for placement in Placement::placements_non_zoro() {
            assert_eq!(
                board.is_placeable(placement),
                reachables.contains(placement),
                "Failed at ({}, {})",
                placement.axis_x(),
                placement.rot()
            );
        }
    }

    #[test]
    fn is_placeable_upper_6() {
        let board: Board = [11, 13, 11, 12, 11, 11].into();
        let unreachables = [
            Placement::new(1, 0),
            Placement::new(2, 0),
            Placement::new(1, 1),
            Placement::new(2, 1),
            Placement::new(1, 2),
            Placement::new(2, 2),
            Placement::new(2, 3),
            Placement::new(3, 3),
            Placement::new(4, 2),
        ];

        for placement in Placement::placements_non_zoro() {
            assert_eq!(
                board.is_placeable(placement),
                !unreachables.contains(placement),
                "Failed at ({}, {})",
                placement.axis_x(),
                placement.rot()
            );
        }
    }

    #[test]
    fn is_placeable_upper_7() {
        let board: Board = [11, 13, 11, 13, 11, 11].into();
        let reachables = [Placement::new(3, 0), Placement::new(3, 2)];

        for placement in Placement::placements_non_zoro() {
            assert_eq!(
                board.is_placeable(placement),
                reachables.contains(placement),
                "Failed at ({}, {})",
                placement.axis_x(),
                placement.rot()
            );
        }
    }

    #[test]
    fn is_placeable_upper_8() {
        let board: Board = [10, 12, 10, 10, 11, 12].into();
        let unreachables = [Placement::new(2, 2), Placement::new(6, 2)];

        for placement in Placement::placements_non_zoro() {
            assert_eq!(
                board.is_placeable(placement),
                !unreachables.contains(placement),
                "Failed at ({}, {})",
                placement.axis_x(),
                placement.rot()
            );
        }
    }

    #[test]
    fn is_placeable_corner() {
        let board: Board = [10, 10, 10, 12, 11, 12].into();
        let reachables = [
            Placement::new(1, 0),
            Placement::new(1, 1),
            Placement::new(1, 2),
            Placement::new(2, 0),
            Placement::new(2, 1),
            Placement::new(2, 2),
            Placement::new(2, 3),
            Placement::new(3, 0),
            Placement::new(3, 2),
            Placement::new(3, 3),
        ];

        for placement in Placement::placements_non_zoro() {
            assert_eq!(
                board.is_placeable(placement),
                reachables.contains(placement),
                "Failed at ({}, {})",
                placement.axis_x(),
                placement.rot()
            );
        }
    }
}
