use self::placement::{PLACEMENTS, PLACEMENTS_ZORO};
use crate::board::WIDTH;

pub mod basic_frame;
pub mod placeable;
mod placement;
pub mod real_frame;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Placement(usize, usize);

impl Placement {
    pub const fn new(x: usize, r: usize) -> Self {
        Self(x, r)
    }

    pub const fn axis_x(&self) -> usize {
        self.0
    }

    pub const fn child_x(&self) -> usize {
        debug_assert!(self.is_valid());

        match self.rot() {
            0 | 2 => self.0,
            1 => self.0 + 1,
            3 => self.0 - 1,
            _ => unreachable!(),
        }
    }

    pub const fn rot(&self) -> usize {
        self.1
    }

    pub const fn is_valid(&self) -> bool {
        if self.axis_x() < 1 || self.axis_x() > WIDTH || self.rot() > 3 {
            return false;
        }
        if self.rot() == 1 && self.axis_x() == WIDTH {
            return false;
        }
        if self.rot() == 3 && self.axis_x() == 1 {
            return false;
        }
        true
    }

    pub const fn placements_non_zoro() -> &'static [Placement; 22] {
        PLACEMENTS
    }

    pub const fn placements_zoro() -> &'static [Placement; 11] {
        PLACEMENTS_ZORO
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn child_x() {
        let placement = Placement::new(3, 0);
        assert_eq!(placement.axis_x(), 3);
        assert_eq!(placement.child_x(), 3);
        assert_eq!(placement.rot(), 0);

        let placement = Placement::new(4, 1);
        assert_eq!(placement.axis_x(), 4);
        assert_eq!(placement.child_x(), 5);
        assert_eq!(placement.rot(), 1);

        let placement = Placement::new(1, 2);
        assert_eq!(placement.axis_x(), 1);
        assert_eq!(placement.child_x(), 1);
        assert_eq!(placement.rot(), 2);

        let placement = Placement::new(6, 3);
        assert_eq!(placement.axis_x(), 6);
        assert_eq!(placement.child_x(), 5);
        assert_eq!(placement.rot(), 3);
    }

    #[test]
    fn is_valid() {
        for x in 0..=WIDTH + 1 {
            for r in 0..=4 {
                assert_eq!(
                    Placement::new(x, r).is_valid(),
                    PLACEMENTS.contains(&Placement::new(x, r))
                );
            }
        }
    }
}
