#[derive(Default)]
pub struct Chain {
    chain: u32,
    score: u32,
    frame: u32,
    quick: bool,
}

impl Chain {
    pub fn new(chain: u32, score: u32, frame: u32, quick: bool) -> Self {
        Self {
            chain,
            score,
            frame,
            quick,
        }
    }

    pub fn chain(&self) -> u32 {
        self.chain
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn frame(&self) -> u32 {
        self.frame
    }

    pub fn quick(&self) -> bool {
        self.quick
    }
}
