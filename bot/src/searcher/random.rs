use core::{board::Board, placement::Placement, player_state::PlayerState};

use rand::seq::SliceRandom;

use super::Searcher;
use crate::{decision::Decision, evaluator::Evaluator};

pub struct RandomSearcher;

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

#[cfg(test)]
mod tests {
    use core::{
        color::PuyoColor::*,
        tumo::{Tumo, Tumos},
    };

    use super::*;
    use crate::evaluator::BUILD;

    #[test]
    fn search_returns_valid_placement() {
        let boards: [Board; 8] = [
            [0, 0, 0, 0, 0, 0].into(),
            [11, 11, 11, 11, 11, 11].into(),
            [12, 12, 11, 12, 12, 12].into(),
            [11, 11, 11, 13, 11, 11].into(),
            [11, 13, 11, 11, 11, 11].into(),
            [11, 12, 11, 13, 11, 11].into(),
            [11, 13, 11, 12, 11, 11].into(),
            [11, 13, 11, 13, 11, 11].into(),
        ];
        let tumos_pattern: [Tumos; 2] = [
            Tumos::new(&vec![Tumo::new(RED, GREEN)]),
            Tumos::new(&vec![Tumo::new_zoro(BLUE)]),
        ];

        for board in &boards {
            for tumos in &tumos_pattern {
                let player_state = PlayerState::new(board.clone(), tumos.clone(), 0, 0, 0, 0, 0, 0);
                let decision = RandomSearcher::search(&player_state, &BUILD, None);

                assert!(!decision.placements.is_empty());
                assert!(board.is_placeable(decision.placements.first().unwrap()));
            }
        }
    }
}
