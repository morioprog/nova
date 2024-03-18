use std::fmt::Debug;

pub trait Color: Clone + Copy + PartialEq<Self> + Debug {
    fn to_char(&self) -> char;
    fn is_normal_color(&self) -> bool;
}
