use core::{chain::Chain, placement::Placement, player_state::PlayerState, tumo::Tumo};

use eval::Evaluator;

#[derive(Clone, Default)]
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

pub fn max_by_chain(nodes: &Vec<Node>) -> &Node {
    nodes
        .iter()
        .max_by(|a, b| {
            let a_score = a.chain.as_ref().unwrap().score();
            let b_score = b.chain.as_ref().unwrap().score();
            a_score.cmp(&b_score)
        })
        .unwrap()
}

pub fn sort_by_eval(nodes: &mut Vec<Node>) {
    nodes.sort_by(|a, b| {
        let a_score = a.eval_score.unwrap();
        let b_score = b.eval_score.unwrap();
        b_score.cmp(&a_score)
    })
}
