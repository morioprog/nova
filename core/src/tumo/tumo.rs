use crate::color::{Color, PuyoColor, RealColor};

pub type Tumo = Pair<PuyoColor>;
pub type RealTumo = Pair<RealColor>;

#[derive(PartialEq, Debug)]
pub struct Pair<C: Color> {
    axis: C,
    child: C,
}

impl<C: Color> Pair<C> {
    pub fn new(axis: C, child: C) -> Self {
        Self { axis, child }
    }

    pub fn is_zoro(&self) -> bool {
        self.axis == self.child
    }

    pub fn is_valid(&self) -> bool {
        self.axis.is_normal_color() && self.child.is_normal_color()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::PuyoColor::{BLUE, RED, WALL};

    #[test]
    fn new() {
        let tumo = Tumo::new(BLUE, RED);
        assert_eq!(tumo.axis, BLUE);
        assert_eq!(tumo.child, RED);
    }

    #[test]
    fn is_zoro() {
        assert_eq!(Tumo::new(BLUE, BLUE).is_zoro(), true);
        assert_eq!(Tumo::new(RED, BLUE).is_zoro(), false);
    }

    #[test]
    fn is_valid() {
        assert_eq!(Tumo::new(BLUE, BLUE).is_valid(), true);
        assert_eq!(Tumo::new(RED, BLUE).is_valid(), true);
        assert_eq!(Tumo::new(BLUE, WALL).is_valid(), false);
        assert_eq!(Tumo::new(WALL, WALL).is_valid(), false);
    }
}
