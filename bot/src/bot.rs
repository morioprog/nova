use core::player_state::PlayerState;
use std::time::Instant;

use crate::{decision::DecisionWithoutElapsed, Decision};

pub trait Bot {
    fn name(&self) -> &'static str;
    fn think_internal_1p(&self, player_state: &PlayerState) -> DecisionWithoutElapsed;
    fn think_internal_2p(
        &self,
        player_state_1p: &PlayerState,
        player_state_2p: &PlayerState,
    ) -> DecisionWithoutElapsed;

    fn think_1p(&self, player_state: &PlayerState) -> Decision {
        let start = Instant::now();
        let decision = self
            .think_internal_1p(player_state)
            .with_elapsed(start.elapsed());
        assert!(!decision.placements.is_empty());

        decision
    }
    fn think_2p(&self, player_state_1p: &PlayerState, player_state_2p: &PlayerState) -> Decision {
        let start = Instant::now();
        let decision = self
            .think_internal_2p(player_state_1p, player_state_2p)
            .with_elapsed(start.elapsed());
        assert!(!decision.placements.is_empty());

        decision
    }
}
