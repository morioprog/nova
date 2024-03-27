use core::placement::Placement;
use std::time::Duration;

#[derive(Default)]
pub struct Decision {
    pub placements: Vec<Placement>,
    pub logging: Option<String>,
    pub elapsed: Duration,
}
