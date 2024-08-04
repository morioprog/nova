use core::player_state::PlayerState;
use std::time::Instant;

use log::warn;

use crate::{
    chain_picker::{enumerate_fireable_chains, strategies::*, ChainPicker},
    decision::{Decision, DecisionWithElapsed},
    evaluator::{select_best_evaluator, Evaluator},
    searcher::{ChokudaiSearcher, Searcher},
};

#[derive(Default)]
pub struct Nova {
    custom_evaluator: Option<Evaluator>,
    custom_beam_search_depth: Option<usize>,
    custom_beam_search_width: Option<usize>,
}

impl Nova {
    pub fn with_evaluator(evaluator: Evaluator) -> Self {
        Self {
            custom_evaluator: Some(evaluator),
            custom_beam_search_depth: None,
            custom_beam_search_width: None,
        }
    }

    pub fn with_custom_params(depth: usize, width: usize) -> Self {
        Self {
            custom_evaluator: None,
            custom_beam_search_depth: Some(depth),
            custom_beam_search_width: Some(width),
        }
    }

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

        let evaluator = self
            .custom_evaluator
            .clone()
            .unwrap_or(select_best_evaluator(player_state_1p, player_state_2p));
        let build_decision = ChokudaiSearcher::search(
            player_state_1p,
            &evaluator,
            self.debug_think_frame(think_frame),
        );

        build_decision
    }

    // TODO: super ad-hoc!!
    fn debug_think_frame(&self, fallback: Option<u32>) -> Option<u32> {
        if self.custom_beam_search_depth.is_none() || self.custom_beam_search_width.is_none() {
            fallback
        } else {
            let depth = self.custom_beam_search_depth.unwrap();
            let width = self.custom_beam_search_width.unwrap();
            Some((1000000 + width * 1000 + depth) as u32)
        }
    }
}
