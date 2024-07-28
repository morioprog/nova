use core::{chain::Chain, placement::Placement, player_state::PlayerState, tumo::Tumo};

use crate::evaluator::Evaluator;

#[derive(Clone, Default)]
pub(super) struct Node {
    pub eval_score: i32,
    pub chain: Chain,
    pub player_state: PlayerState,
    pub placements: Vec<Placement>,
}

impl Node {
    pub fn from_player_state(
        player_state: &PlayerState,
        placements: &[Placement],
        evaluator: &Evaluator,
    ) -> Self {
        let mut new_player_state = player_state.clone();
        let chain = new_player_state.board.simulate();
        new_player_state.frame += chain.frame();

        Self {
            eval_score: evaluator.evaluate(&new_player_state),
            chain: chain,
            player_state: new_player_state,
            placements: placements.into(),
        }
    }

    pub fn place_tumo(&self, tumo: &Tumo, placement: &Placement, evaluator: &Evaluator) -> Self {
        let mut new_player_state = self.player_state.clone();
        let frame = new_player_state.board.place_tumo(tumo, placement).unwrap();
        new_player_state.frame += frame;

        let mut new_placements = self.placements.clone();
        new_placements.push(*placement);

        Self::from_player_state(&new_player_state, &new_placements, evaluator)
    }
}

pub(super) fn sort_by_eval(nodes: &mut Vec<Node>) {
    nodes.sort_by(|a, b| b.eval_score.cmp(&a.eval_score))
}
