use core::{chain::Chain, placement::Placement, player_state::PlayerState, tumo::Tumo};

use crate::{evaluator::Evaluator, DetailedPlayerState};

#[derive(Clone, Default)]
pub(super) struct Node {
    pub eval_score: i32,
    pub chain: Chain,
    pub player_state: DetailedPlayerState,
    pub placements: Vec<Placement>,
}

impl Node {
    pub fn from_player_state(
        player_state: &PlayerState,
        placements: &[Placement],
        evaluator: &Evaluator,
    ) -> Self {
        Self::from_detailed_player_state(&player_state.clone().into(), placements, evaluator)
    }

    pub fn from_detailed_player_state(
        player_state: &DetailedPlayerState,
        placements: &[Placement],
        evaluator: &Evaluator,
    ) -> Self {
        let mut new_player_state = player_state.clone();
        let chain = new_player_state.board.simulate();
        new_player_state.frame += chain.frame();
        new_player_state.frame_since_control_start += chain.frame();
        new_player_state.frame_by_chain += chain.frame();

        Self {
            eval_score: evaluator.evaluate(&new_player_state),
            chain: chain,
            player_state: new_player_state,
            placements: placements.into(),
        }
    }

    pub fn place_tumo(&self, tumo: &Tumo, placement: &Placement, evaluator: &Evaluator) -> Self {
        let mut new_player_state = self.player_state.clone();
        new_player_state.frame_by_chigiri += new_player_state.board.chigiri_frames(placement);
        let place_frame = new_player_state.board.place_tumo(tumo, placement).unwrap();
        new_player_state.frame += place_frame;
        new_player_state.frame_since_control_start += place_frame;

        let mut new_placements = self.placements.clone();
        new_placements.push(*placement);

        Self::from_detailed_player_state(&new_player_state, &new_placements, evaluator)
    }
}

pub(super) fn sort_by_eval(nodes: &mut Vec<Node>) {
    nodes.sort_by(|a, b| b.eval_score.cmp(&a.eval_score))
}
