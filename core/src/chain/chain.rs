#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Chain(u32, u32, u32);

impl Chain {
    pub const fn new(chain: u32, score: u32, frame: u32) -> Self {
        Self(chain, score, frame)
    }

    pub const fn chain(&self) -> u32 {
        self.0
    }

    pub const fn score(&self) -> u32 {
        self.1
    }

    pub const fn frame(&self) -> u32 {
        self.2
    }
}
