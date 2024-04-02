use core::{chain::Chain, placement::Placement, player_state::PlayerState, tumo::Tumo};

use eval::Evaluator;

pub(super) struct Node {
    pub eval_score: Option<i32>,
    pub chain: Option<Chain>,
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

        if chain.chain() > 0 {
            Self {
                eval_score: None,
                chain: Some(chain),
                player_state: new_player_state,
                placements: placements.into(),
            }
        } else {
            Self {
                eval_score: Some(evaluator.evaluate(player_state)),
                chain: None,
                player_state: new_player_state,
                placements: placements.into(),
            }
        }
    }

    pub fn place_tumo(&self, tumo: &Tumo, placement: &Placement, evaluator: &Evaluator) -> Self {
        let mut new_player_state = self.player_state.clone();
        let _frame = new_player_state.board.place_tumo(tumo, placement);

        let mut new_placements = self.placements.clone();
        new_placements.push(*placement);

        Self::from_player_state(&new_player_state, &new_placements, evaluator)
    }
}
