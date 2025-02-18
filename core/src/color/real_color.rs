use super::Color;

/// [Color] impl for an actual game.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum RealColor {
    #[default]
    EMPTY = 0,
    WALL = 1,
    OJAMA = 2,
    RED = 3,
    GREEN = 4,
    BLUE = 5,
    YELLOW = 6,
    PURPLE = 7,
}

const NORMAL_COLORS: &[RealColor; 5] = &[
    RealColor::RED,
    RealColor::GREEN,
    RealColor::BLUE,
    RealColor::YELLOW,
    RealColor::PURPLE,
];

impl Color for RealColor {
    fn is_normal_color(&self) -> bool {
        ((*self as u8) + 1) & 0b1100 != 0
    }
}

impl RealColor {
    pub const fn normal_colors() -> &'static [RealColor; 5] {
        NORMAL_COLORS
    }
}

impl From<u8> for RealColor {
    fn from(value: u8) -> Self {
        match value {
            b' ' | b'.' => RealColor::EMPTY,
            b'O' | b'o' | b'@' => RealColor::OJAMA,
            b'#' => RealColor::WALL,
            b'R' | b'r' => RealColor::RED,
            b'G' | b'g' => RealColor::GREEN,
            b'B' | b'b' => RealColor::BLUE,
            b'Y' | b'y' => RealColor::YELLOW,
            b'P' | b'p' => RealColor::PURPLE,
            _ => unreachable!(),
        }
    }
}

impl From<char> for RealColor {
    fn from(value: char) -> Self {
        match value {
            ' ' | '.' => RealColor::EMPTY,
            'O' | 'o' | '@' => RealColor::OJAMA,
            '#' => RealColor::WALL,
            'R' | 'r' => RealColor::RED,
            'G' | 'g' => RealColor::GREEN,
            'B' | 'b' => RealColor::BLUE,
            'Y' | 'y' => RealColor::YELLOW,
            'P' | 'p' => RealColor::PURPLE,
            _ => unreachable!(),
        }
    }
}

impl From<RealColor> for u8 {
    fn from(value: RealColor) -> Self {
        match value {
            RealColor::EMPTY => b' ',
            RealColor::WALL => b'#',
            RealColor::OJAMA => b'O',
            RealColor::RED => b'R',
            RealColor::GREEN => b'G',
            RealColor::BLUE => b'B',
            RealColor::YELLOW => b'Y',
            RealColor::PURPLE => b'P',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_byte() {
        assert_eq!(u8::from(RealColor::EMPTY), b' ');
        assert_eq!(u8::from(RealColor::WALL), b'#');
        assert_eq!(u8::from(RealColor::RED), b'R');
        assert_eq!(u8::from(RealColor::PURPLE), b'P');
    }

    #[test]
    fn from_char() {
        assert_eq!(RealColor::from(' '), RealColor::EMPTY);
        assert_eq!(RealColor::from('#'), RealColor::WALL);
        assert_eq!(RealColor::from('R'), RealColor::RED);
        assert_eq!(RealColor::from('p'), RealColor::PURPLE);
    }

    #[test]
    fn is_normal_color() {
        assert_eq!(RealColor::EMPTY.is_normal_color(), false);
        assert_eq!(RealColor::WALL.is_normal_color(), false);
        assert_eq!(RealColor::OJAMA.is_normal_color(), false);
        assert_eq!(RealColor::RED.is_normal_color(), true);
        assert_eq!(RealColor::GREEN.is_normal_color(), true);
        assert_eq!(RealColor::BLUE.is_normal_color(), true);
        assert_eq!(RealColor::YELLOW.is_normal_color(), true);
        assert_eq!(RealColor::PURPLE.is_normal_color(), true);
    }
}
