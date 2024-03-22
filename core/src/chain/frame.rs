// ref. https://puyo-camp.jp/posts/71019

const CHAIN_FRAMES: &'static [u32; 13] = &[55, 80, 82, 84, 86, 88, 90, 92, 94, 96, 98, 100, 102];

pub const fn chain_frames(max_drops: u16) -> u32 {
    debug_assert!(max_drops <= 12);
    CHAIN_FRAMES[max_drops as usize]
}
