mod puyo_color;
mod real_color;

use std::fmt::Debug;

pub use self::{puyo_color::PuyoColor, real_color::RealColor};

pub trait Color: Clone + Copy + PartialEq<Self> + Debug + Default + From<u8> + Into<u8> {
    fn is_normal_color(&self) -> bool;
}
