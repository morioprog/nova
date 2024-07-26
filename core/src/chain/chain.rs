#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Debug)]
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

impl std::ops::Add for Chain {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}
