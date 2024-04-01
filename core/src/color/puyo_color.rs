use std::mem;

use rand::Rng;

use super::Color;

/// [Color] impl for a simulation purpose.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum PuyoColor {
    #[default]
    EMPTY = 0,
    WALL = 1,
    OJAMA = 2,
    IRON = 3,
    RED = 4,
    GREEN = 5,
    BLUE = 6,
    YELLOW = 7,
}

impl Color for PuyoColor {
    fn to_char(&self) -> char {
        match *self {
            PuyoColor::EMPTY => ' ',
            PuyoColor::WALL => '#',
            PuyoColor::OJAMA => 'O',
            PuyoColor::IRON => '&',
            PuyoColor::RED => 'R',
            PuyoColor::GREEN => 'G',
            PuyoColor::BLUE => 'B',
            PuyoColor::YELLOW => 'Y',
        }
    }

    fn is_normal_color(&self) -> bool {
        (*self as u8) & 0b100 != 0
    }
}

impl PuyoColor {
    pub fn random_normal_color() -> Self {
        unsafe { mem::transmute(rand::thread_rng().gen_range(4u8..8u8)) }
    }
}

impl From<u8> for PuyoColor {
    fn from(value: u8) -> Self {
        match value {
            b' ' | b'.' => PuyoColor::EMPTY,
            b'O' | b'o' | b'@' => PuyoColor::OJAMA,
            b'#' => PuyoColor::WALL,
            b'&' => PuyoColor::IRON,
            b'R' | b'r' => PuyoColor::RED,
            b'G' | b'g' => PuyoColor::GREEN,
            b'B' | b'b' => PuyoColor::BLUE,
            b'Y' | b'y' => PuyoColor::YELLOW,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_char() {
        assert_eq!(PuyoColor::EMPTY.to_char(), ' ');
        assert_eq!(PuyoColor::WALL.to_char(), '#');
        assert_eq!(PuyoColor::RED.to_char(), 'R');
        assert_eq!(PuyoColor::BLUE.to_char(), 'B');
    }

    #[test]
    fn is_normal_color() {
        assert_eq!(PuyoColor::EMPTY.is_normal_color(), false);
        assert_eq!(PuyoColor::WALL.is_normal_color(), false);
        assert_eq!(PuyoColor::OJAMA.is_normal_color(), false);
        assert_eq!(PuyoColor::IRON.is_normal_color(), false);
        assert_eq!(PuyoColor::RED.is_normal_color(), true);
        assert_eq!(PuyoColor::GREEN.is_normal_color(), true);
        assert_eq!(PuyoColor::BLUE.is_normal_color(), true);
        assert_eq!(PuyoColor::YELLOW.is_normal_color(), true);
    }

    #[test]
    fn random_normal_color() {
        for _ in 0..100 {
            assert!(PuyoColor::random_normal_color().is_normal_color());
        }
    }
}
