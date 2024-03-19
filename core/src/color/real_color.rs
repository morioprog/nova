use super::Color;

/// [Color] impl for an actual game.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum RealColor {
    EMPTY = 0,
    WALL = 1,
    OJAMA = 2,
    RED = 3,
    GREEN = 4,
    BLUE = 5,
    YELLOW = 6,
    PURPLE = 7,
}

impl Default for RealColor {
    fn default() -> Self {
        RealColor::EMPTY
    }
}

impl Color for RealColor {
    fn to_char(&self) -> char {
        match self {
            &RealColor::EMPTY => ' ',
            &RealColor::WALL => '#',
            &RealColor::OJAMA => 'O',
            &RealColor::RED => 'R',
            &RealColor::GREEN => 'G',
            &RealColor::BLUE => 'B',
            &RealColor::YELLOW => 'Y',
            &RealColor::PURPLE => 'P',
        }
    }

    fn is_normal_color(&self) -> bool {
        ((*self as u8) + 1) & 0b1100 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_char() {
        assert_eq!(RealColor::EMPTY.to_char(), ' ');
        assert_eq!(RealColor::WALL.to_char(), '#');
        assert_eq!(RealColor::RED.to_char(), 'R');
        assert_eq!(RealColor::PURPLE.to_char(), 'P');
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
