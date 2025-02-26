// ref. https://puyo-camp.jp/posts/71019

use super::Placement;
use crate::board::{Board, WIDTH};

const HORIZONTAL_MOVE_FRAMES: &[u32; WIDTH + 1] = &[6, 4, 2, 0, 2, 4, 6];
// Note. 26, 24, 22 are just there to avoid out-of-bound access.
const VERTICAL_MOVE_FRAMES: &[u32; 16] = &[
    52, 50, 48, 46, 44, 42, 40, 38, 36, 34, 32, 30, 28, 26, 24, 22,
];
// Note. 50, 52, 54, 56 are just there to avoid out-of-bound access.
const CHIGIRI_FRAMES: &[u32; 16] = &[
    0, 19, 24, 28, 31, 34, 37, 40, 42, 44, 46, 48, 50, 52, 54, 56,
];

/// Please pass `x` of the destination cell as a parameter.
pub const fn horizontal_move_frames(x: usize) -> u32 {
    debug_assert!(1 <= x && x <= WIDTH);
    HORIZONTAL_MOVE_FRAMES[x]
}

/// Please pass `y` of the destination cell as a parameter.
pub const fn vertical_move_frames(y: usize) -> u32 {
    debug_assert!(1 <= y && y <= 15);
    VERTICAL_MOVE_FRAMES[y]
}

pub const fn chigiri_frames(y_diff: usize) -> u32 {
    debug_assert!(y_diff < 16);
    CHIGIRI_FRAMES[y_diff]
}

impl Board {
    /// Not considering "wall jumping".
    pub fn basic_place_frames(&self, placement: &Placement) -> u32 {
        self.move_frames(placement) + self.chigiri_frames(placement)
    }

    pub fn move_frames(&self, placement: &Placement) -> u32 {
        let heights = self.height_array();
        let axis_height = heights[placement.axis_x()];
        let child_height = heights[placement.child_x()];

        let y = match placement.rot() {
            0 => axis_height + 1,
            2 => axis_height + 2,
            1 | 3 => axis_height.max(child_height) + 1,
            _ => unreachable!(),
        };

        horizontal_move_frames(placement.axis_x()) + vertical_move_frames(y)
    }

    pub fn chigiri_frames(&self, placement: &Placement) -> u32 {
        let heights = self.height_array();
        let axis_height = heights[placement.axis_x()];
        let child_height = heights[placement.child_x()];

        let y_diff = axis_height.abs_diff(child_height);

        chigiri_frames(y_diff)
    }
}
