const CHAIN_BONUS: &[u32; 20] = &[
    0, 0, 8, 16, 32, 64, 96, 128, 160, 192, 224, 256, 288, 320, 352, 384, 416, 448, 480, 512,
];
const COLOR_BONUS: &[u32; 6] = &[0, 0, 3, 6, 12, 24];
const CONN_BONUS: &[u32; 12] = &[0, 0, 0, 0, 0, 2, 3, 4, 5, 6, 7, 10];

pub const fn chain_bonus(nth_chain: usize) -> u32 {
    debug_assert!(nth_chain <= 19);
    CHAIN_BONUS[nth_chain]
}
pub const fn color_bonus(num_colors: usize) -> u32 {
    debug_assert!(num_colors <= 5);
    COLOR_BONUS[num_colors]
}
pub const fn conn_bonus(num_conns: usize) -> u32 {
    CONN_BONUS[if num_conns > 11 { 11 } else { num_conns }]
}
