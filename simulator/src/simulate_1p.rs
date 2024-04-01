use core::{player_state::PlayerState, tumo::Tumos};

use bot::Bot;

use crate::simulate_result::simulate_1p_result::Simulate1PResult;

pub fn simulate_1p(bot: impl Bot) -> Simulate1PResult {
    // TODO: pass visible as parameter
    let visible = 3;
    let mut player_state = {
        let mut tumos = Tumos::new_random();
        tumos.set_visible(visible); // till next2
        PlayerState::initial_state(tumos)
    };

    let mut score = 0;
    let mut decisions = vec![];

    // TODO: pass 50 as parameter
    for _ in 0..50 {
        let decision = bot.think_1p(&player_state);

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
        score += chain.score();
        // TODO: pass 80000 as parameter
        if chain.score() >= 80000 {
            break;
        }

        if player_state.board.is_dead() {
            break;
        }

        player_state.tumos.rotate();
    }

    Simulate1PResult {
        score,
        visible,
        decisions,
        tumos: player_state.tumos,
    }
}
