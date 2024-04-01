use core::placement::Placement;
use std::time::Duration;

#[derive(Clone, Default)]
pub struct Decision {
    pub placements: Vec<Placement>,
    pub logging: Option<String>,
    pub elapsed: Duration,
}
