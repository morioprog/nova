use core::player_state::PlayerState;
use std::time::Instant;

use log::warn;

use crate::{
    chain_picker::{enumerate_fireable_chains, ChainPicker, Houwa},
    decision::{Decision, DecisionWithElapsed},
    evaluator::select_best_evaluator,
    searcher::{BeamSearcher, Searcher},
};

pub struct Nova;

impl Nova {
    pub fn think(
        &self,
        player_state_1p: &PlayerState,
        player_state_2p: Option<&PlayerState>,
        think_frame: Option<u32>,
    ) -> DecisionWithElapsed {
        let start = Instant::now();
        let decision = self
            .think_internal(player_state_1p, player_state_2p, think_frame)
            .with_elapsed(start.elapsed());

        if decision.placements.is_empty() {
            warn!("Nova returned Decision with empty Placement!");
            return Decision::fallback().with_elapsed(start.elapsed());
        }

        decision
    }

    fn think_internal(
        &self,
        player_state_1p: &PlayerState,
        player_state_2p: Option<&PlayerState>,
        think_frame: Option<u32>,
    ) -> Decision {
        // TODO: OpeningMatcher

        let chain_decisions = enumerate_fireable_chains(player_state_1p);
        macro_rules! try_pick_chain {
            ($($chain_picker:ty),*) => {
                $(
                    if let Some(decision) = <$chain_picker>::pick_chain(player_state_1p, player_state_2p, &chain_decisions) {
                        return decision;
                    }
                )*
            };
        }
        try_pick_chain!(Houwa);

        let evaluator = select_best_evaluator(player_state_1p, player_state_2p);
        let build_decision = BeamSearcher::search(player_state_1p, &evaluator, think_frame);

        build_decision
    }
}
