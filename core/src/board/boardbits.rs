cfg_if::cfg_if! {
    if #[cfg(all(target_arch = "x86_64"))] {
        mod x86_64;
        pub(crate) use self::x86_64::BoardBits;
    } else {
        todo!()
        // mod base;
        // pub(crate) use self::base::BoardBits;
    }
}

pub(crate) trait BoardManipulation {
    // constructor
    fn zero() -> Self;
    fn wall() -> Self;
    fn onebit(x: usize, y: usize) -> Self;

    fn get(&self, x: usize, y: usize) -> bool;
}
