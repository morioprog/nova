use core::{
    board::Board, chain::Chain, placement::Placement, puyop::construct_sim1p_url, tumo::Tumos,
};

use bot::DecisionWithElapsed;

pub struct Simulate1PResult {
    pub score: u32,
    pub max_chain: Chain,
    pub visible: usize,
    pub decisions: Vec<DecisionWithElapsed>,
    pub tumos: Tumos,
}

impl Simulate1PResult {
    pub fn create_puyop_url(&self) -> String {
        construct_sim1p_url(
            &Board::new(),
            &self.tumos,
            &self
                .decisions
                .iter()
                .map(|d| *d.placements.first().unwrap())
                .collect::<Vec<Placement>>(),
        )
    }
}
