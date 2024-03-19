mod tumo;
mod tumos;

pub use self::{
    tumo::{RealTumo, Tumo},
    tumos::{RealTumos, Tumos},
};

/// Loop length of tumo.
pub const TUMO_LOOP: usize = 128;
