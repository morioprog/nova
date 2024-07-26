use core::{chain::Chain, placement::Placement};
use std::time::Duration;

#[derive(Clone, Default)]
pub struct DecisionWithElapsed {
    pub placements: Vec<Placement>,
    pub chain: Chain,
    pub logging: Option<String>,
    pub elapsed: Duration,
}

#[derive(Clone, Default)]
pub struct Decision {
    pub placements: Vec<Placement>,
    pub chain: Chain,
    pub logging: Option<String>,
}

impl Decision {
    pub fn with_elapsed(self, elapsed: Duration) -> DecisionWithElapsed {
        DecisionWithElapsed {
            placements: self.placements,
            chain: self.chain,
            logging: self.logging,
            elapsed,
        }
    }
}
