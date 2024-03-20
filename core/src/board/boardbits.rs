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

    fn mask(&self, mask: Self) -> Self;
    fn mask_12(&self) -> Self;
    fn mask_13(&self) -> Self;
    fn not_mask(&self, mask: Self) -> Self;
    fn not_mask_12(&self) -> Self;
    fn not_mask_13(&self) -> Self;

    fn get(&self, x: usize, y: usize) -> u8;
}
