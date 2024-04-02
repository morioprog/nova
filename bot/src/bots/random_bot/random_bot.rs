use core::{board::Board, placement::Placement, player_state::PlayerState};

use rand::seq::SliceRandom;

use crate::{decision::DecisionWithoutElapsed, Bot};

pub struct RandomBot {}

impl Bot for RandomBot {
    fn name(&self) -> &'static str {
        "RandomBot"
    }

    fn think_internal_1p(&self, player_state: &PlayerState) -> DecisionWithoutElapsed {
        DecisionWithoutElapsed {
            placements: vec![Self::random_valid_placement(
                &player_state.board,
                player_state.tumos[0].is_zoro(),
            )],
            ..DecisionWithoutElapsed::default()
        }
    }

    fn think_internal_2p(
        &self,
        player_state_1p: &PlayerState,
        _player_state_2p: &PlayerState,
    ) -> DecisionWithoutElapsed {
        self.think_internal_1p(player_state_1p)
    }
}

impl RandomBot {
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

    #[test]
    fn think_returns_valid_placement() {
        let bot = RandomBot {};
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
                let decision = bot.think_internal_1p(&PlayerState {
                    board: board.clone(),
                    tumos: tumos.clone(),
                    ..PlayerState::zero()
                });
                assert!(!decision.placements.is_empty());
                assert!(board.is_placeable(decision.placements.first().unwrap()));
            }
        }
    }
}
