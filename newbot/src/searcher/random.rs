use core::{board::Board, placement::Placement, player_state::PlayerState};

use rand::seq::SliceRandom;

use super::Searcher;
use crate::{decision::Decision, evaluator::Evaluator};

pub(crate) struct RandomSearcher;

impl Searcher for RandomSearcher {
    fn search(
        player_state: &PlayerState,
        _evaluator: &Evaluator,
        _think_frame: Option<u32>,
    ) -> Decision {
        let placement =
            Self::random_valid_placement(&player_state.board, player_state.tumos[0].is_zoro());
        Decision {
            placements: vec![placement],
            ..Decision::default()
        }
    }
}

impl RandomSearcher {
    fn random_valid_placement(board: &Board, is_zoro: bool) -> Placement {
        **board
            .valid_placements(is_zoro)
            .choose(&mut rand::thread_rng())
            .unwrap()
    }
}
