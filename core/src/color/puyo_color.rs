use super::Color;

/// [Color] impl for a simulation purpose.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PuyoColor {
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
        match self {
            &PuyoColor::EMPTY => ' ',
            &PuyoColor::WALL => '#',
            &PuyoColor::OJAMA => 'O',
            &PuyoColor::IRON => '&',
            &PuyoColor::RED => 'R',
            &PuyoColor::GREEN => 'G',
            &PuyoColor::BLUE => 'B',
            &PuyoColor::YELLOW => 'Y',
        }
    }

    fn is_normal_color(&self) -> bool {
        (*self as u8) & 0b100 != 0
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
}
