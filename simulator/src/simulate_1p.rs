use core::{player_state::PlayerState, tumo::Tumos};

use bot::Nova;

use crate::simulate_result::simulate_1p_result::Simulate1PResult;

pub fn simulate_1p(nova: Nova, tumos: Option<Tumos>) -> Simulate1PResult {
    // TODO: pass visible as parameter
    let visible = 3;

    let mut player_state = PlayerState::initial_state(tumos.unwrap_or(Tumos::new_random()));
    let mut decisions = vec![];

    // TODO: pass 50 as parameter
    for _ in 0..50 {
        let decision = nova.think(&player_state.limit_visible_tumos(visible), None, None);

        let Some(placement) = decision.placements.first() else {
            panic!("Bot returned empty placement!")
        };

        decisions.push(decision.clone());
        // TODO: just disregard frame?
        player_state
            .board
            .place_tumo(&player_state.tumos[0], placement);
        // TODO: consider drop bonus?

        let chain = player_state.board.simulate();
        player_state.score += chain.score();
        // TODO: pass 70000 as parameter
        if chain.score() >= 70000 {
            break;
        }

        if player_state.board.is_dead() {
            break;
        }

        player_state.tumos.rotate(visible);
    }

    Simulate1PResult {
        score: player_state.score,
        visible,
        decisions,
        tumos: player_state.tumos,
    }
}
