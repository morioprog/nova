mod board;
mod boardbits;

pub use self::board::Board;

/// Board width.
pub const WIDTH: usize = 6;
/// Board height (only visible area).
pub const HEIGHT: usize = 12;

/// Board width (including wall).
pub const ENTIRE_WIDTH: usize = 8;
/// Board height (including wall).
pub const ENTIRE_HEIGHT: usize = 16;
