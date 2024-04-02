use core::placement::Placement;
use std::time::Duration;

#[derive(Clone, Default)]
pub struct Decision {
    pub placements: Vec<Placement>,
    pub logging: Option<String>,
    pub elapsed: Duration,
}

#[derive(Clone, Default)]
pub struct DecisionWithoutElapsed {
    pub placements: Vec<Placement>,
    pub logging: Option<String>,
}

impl DecisionWithoutElapsed {
    pub fn with_elapsed(self, elapsed: Duration) -> Decision {
        Decision {
            placements: self.placements,
            logging: self.logging,
            elapsed,
        }
    }
}
