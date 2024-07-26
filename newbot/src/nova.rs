use core::player_state::PlayerState;
use std::time::Instant;

use log::warn;

use crate::{
    chain_picker::{ChainPicker, Houwa},
    decision::{Decision, DecisionWithElapsed},
    evaluator::{select_best_evaluator, BUILD},
    searcher::{RandomSearcher, Searcher},
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
        _think_frame: Option<u32>,
    ) -> Decision {
        // TODO: OpeningMatcher

        let evaluator = if player_state_2p.is_some() {
            select_best_evaluator(player_state_1p, player_state_2p.unwrap())
        } else {
            BUILD
        };
        let (build_decision, chain_decisions) = RandomSearcher::search(player_state_1p, &evaluator);

        let houwa_decision = Houwa::pick_chain(player_state_1p, player_state_2p, &chain_decisions);
        if houwa_decision.is_some() {
            return houwa_decision.unwrap();
        }

        build_decision
    }
}
