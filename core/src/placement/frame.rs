// ref. https://puyo-camp.jp/posts/71019

use crate::board::WIDTH;

const HORIZONTAL_MOVE_FRAMES: &[u32; WIDTH + 1] = &[6, 4, 2, 0, 2, 4, 6];
// Note. 26, 24, 22 are just there to avoid out-of-bound access.
const VERTICAL_MOVE_FRAMES: &[u32; 15] =
    &[50, 48, 46, 44, 42, 40, 38, 36, 34, 32, 30, 28, 26, 24, 22];
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
    debug_assert!(y <= 14);
    VERTICAL_MOVE_FRAMES[y]
}

pub const fn chigiri_frames(y_diff: usize) -> u32 {
    debug_assert!(y_diff < 16);
    CHIGIRI_FRAMES[y_diff]
}
