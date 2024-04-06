use std::mem;

use rand::Rng;

use super::Color;

/// [Color] impl for a simulation purpose.
#[derive(Clone, Copy, PartialEq, Default)]
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
    fn is_normal_color(&self) -> bool {
        (*self as u8) & 0b100 != 0
    }
}

impl PuyoColor {
    pub fn random_normal_color() -> Self {
        unsafe { mem::transmute(rand::thread_rng().gen_range(4u8..8u8)) }
    }

    pub fn bg_escape_sequence(&self) -> &'static str {
        match *self {
            PuyoColor::EMPTY => "",
            PuyoColor::WALL => "\x1b[48;5;253m",
            PuyoColor::OJAMA => "\x1b[48;5;241m",
            PuyoColor::IRON => "\x1b[48;5;237m",
            PuyoColor::RED => "\x1b[48;5;161m",
            PuyoColor::GREEN => "\x1b[48;5;28m",
            PuyoColor::BLUE => "\x1b[48;5;26m",
            PuyoColor::YELLOW => "\x1b[48;5;226m",
        }
    }
}

impl From<u8> for PuyoColor {
    fn from(value: u8) -> Self {
        match value {
            b' ' | b'.' => PuyoColor::EMPTY,
            b'#' => PuyoColor::WALL,
            b'O' | b'o' | b'@' => PuyoColor::OJAMA,
            b'&' => PuyoColor::IRON,
            b'R' | b'r' => PuyoColor::RED,
            b'G' | b'g' => PuyoColor::GREEN,
            b'B' | b'b' => PuyoColor::BLUE,
            b'Y' | b'y' => PuyoColor::YELLOW,
            _ => unreachable!(),
        }
    }
}

impl From<PuyoColor> for u8 {
    fn from(value: PuyoColor) -> Self {
        match value {
            PuyoColor::EMPTY => b' ',
            PuyoColor::WALL => b'#',
            PuyoColor::OJAMA => b'@',
            PuyoColor::IRON => b'&',
            PuyoColor::RED => b'R',
            PuyoColor::GREEN => b'G',
            PuyoColor::BLUE => b'B',
            PuyoColor::YELLOW => b'Y',
        }
    }
}

impl std::fmt::Display for PuyoColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}  \x1b[0m", self.bg_escape_sequence())
    }
}

impl std::fmt::Debug for PuyoColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c: char = u8::from(*self).into();
        write!(f, "{}{}{}\x1b[0m", self.bg_escape_sequence(), c, c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_byte() {
        assert_eq!(u8::from(PuyoColor::EMPTY), b' ');
        assert_eq!(u8::from(PuyoColor::WALL), b'#');
        assert_eq!(u8::from(PuyoColor::RED), b'R');
        assert_eq!(u8::from(PuyoColor::BLUE), b'B');
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
