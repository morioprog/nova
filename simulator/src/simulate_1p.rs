use core::{chain::Chain, player_state::PlayerState, tumo::Tumos};

use bot::Nova;

use crate::simulate_result::simulate_1p_result::Simulate1PResult;

pub fn simulate_1p(nova: Nova, tumos: Option<Tumos>, think_frame: Option<u32>) -> Simulate1PResult {
    // TODO: pass visible as parameter
    let visible = 3;

    let mut player_state = PlayerState::initial_state(tumos.unwrap_or(Tumos::new_random()));
    let mut decisions = vec![];
    let mut max_chain = Chain::default();

    // TODO: pass 54 as parameter
    for _ in 0..54 {
        let decision = nova.think(
            &player_state.limit_visible_tumos(visible),
            None,
            think_frame,
        );

        let Some(placement) = decision.placements.first() else {
            panic!("Bot returned empty placement!")
        };

        decisions.push(decision.clone());
        let place_frame = player_state
            .board
            .place_tumo(&player_state.tumos[0], placement);
        if let Some(frame) = place_frame {
            player_state.frame += frame;
        } else {
            println!("unplaceable...");
            break;
        }
        // TODO: consider drop bonus?

        let chain = player_state.board.simulate();
        player_state.frame += chain.frame();
        player_state.score += chain.score();
        max_chain = max_chain.max(chain);

        if player_state.board.is_dead() {
            break;
        }

        player_state.tumos.rotate(visible);
    }

    Simulate1PResult {
        score: player_state.score,
        max_chain,
        visible,
        decisions,
        tumos: player_state.tumos,
    }
}
