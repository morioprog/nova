use core::{board::WIDTH, player_state::PlayerState};
use std::time::Instant;

use log::warn;

use crate::{
    chain_picker::{enumerate_fireable_chains, strategies::*, ChainPicker},
    decision::{Decision, DecisionWithElapsed},
    evaluator::{select_best_evaluator, Evaluator, BUILD_MIDGAME},
    searcher::*,
};

#[derive(Default)]
pub struct Nova {
    custom_evaluator: Option<Evaluator>,
}

impl Nova {
    pub fn with_evaluator(evaluator: Evaluator) -> Self {
        Self {
            custom_evaluator: Some(evaluator),
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

        let mut evaluator = self
            .custom_evaluator
            .clone()
            .unwrap_or(select_best_evaluator(player_state_1p, player_state_2p));
        // TODO
        if player_state_1p.board.height_array()[1..=WIDTH]
            .iter()
            .sum::<usize>()
            >= 30
        {
            evaluator = BUILD_MIDGAME;
        }
        let build_decision =
            MonteCarloBeamSearcher::search(player_state_1p, &evaluator, think_frame);

        build_decision
    }
}
