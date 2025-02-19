use crate::color::{Color, PuyoColor, RealColor};

pub type Tumo = Pair<PuyoColor>;
pub type RealTumo = Pair<RealColor>;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Pair<C: Color> {
    axis: C,
    child: C,
}

impl<C: Color> Pair<C> {
    pub fn new(axis: C, child: C) -> Self {
        Self { axis, child }
    }

    pub fn new_zoro(axis: C) -> Self {
        Self { axis, child: axis }
    }

    pub const fn axis(&self) -> C {
        self.axis
    }

    pub const fn child(&self) -> C {
        self.child
    }

    pub fn is_zoro(&self) -> bool {
        self.axis == self.child
    }

    pub fn is_valid(&self) -> bool {
        self.axis.is_normal_color() && self.child.is_normal_color()
    }
}

impl Pair<PuyoColor> {
    pub fn new_random() -> Self {
        let axis = PuyoColor::random_normal_color();
        let child = PuyoColor::random_normal_color();
        Self { axis, child }
    }
}

impl<C: Color + From<char>> From<(char, char)> for Pair<C> {
    fn from(value: (char, char)) -> Self {
        let axis = value.0.into();
        let child = value.1.into();
        Self { axis, child }
    }
}

impl<C: Color + Into<char>> std::fmt::Display for Pair<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let axis_c: char = self.axis.into();
        let child_c: char = self.child.into();
        write!(f, "{}{}", axis_c, child_c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::PuyoColor::*;

    #[test]
    fn new() {
        let tumo = Tumo::new(BLUE, RED);
        assert_eq!(tumo.axis(), BLUE);
        assert_eq!(tumo.child(), RED);
    }

    #[test]
    fn from_chars() {
        assert_eq!(Tumo::from(('r', 'b')), Tumo::new(RED, BLUE));
        assert_eq!(Tumo::from(('y', 'y')), Tumo::new(YELLOW, YELLOW));
    }

    #[test]
    fn new_zoro() {
        let tumo = Tumo::new_zoro(GREEN);
        assert_eq!(tumo.axis(), GREEN);
        assert_eq!(tumo.child(), GREEN);
    }

    #[test]
    fn is_zoro() {
        assert_eq!(Tumo::new(GREEN, GREEN).is_zoro(), true);
        assert_eq!(Tumo::new(RED, BLUE).is_zoro(), false);
    }

    #[test]
    fn is_valid() {
        assert_eq!(Tumo::new(BLUE, BLUE).is_valid(), true);
        assert_eq!(Tumo::new(RED, BLUE).is_valid(), true);
        assert_eq!(Tumo::new(BLUE, WALL).is_valid(), false);
        assert_eq!(Tumo::new(WALL, EMPTY).is_valid(), false);
    }

    #[test]
    fn new_random() {
        for _ in 0..100 {
            assert!(Tumo::new_random().is_valid());
        }
    }

    #[test]
    fn to_string() {
        assert_eq!(Tumo::new(RED, GREEN).to_string(), "rg");
    }
}
