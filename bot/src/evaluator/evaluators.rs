use core::player_state::PlayerState;

use super::Evaluator;

pub(crate) fn select_best_evaluator(
    player_state_1p: &PlayerState,
    player_state_2p: Option<&PlayerState>,
) -> Evaluator {
    let player_state_2p = match player_state_2p {
        Some(state) => state,
        None => return BUILD,
    };

    if player_state_1p.carry_over >= 70 * 30 && player_state_2p.carry_over >= 70 * 30 {
        return ZENKESHI;
    }

    BUILD
}

pub const BUILD: Evaluator = Evaluator {
    name: "build",
    bump: -30,
    dent: -30,
    dead_cells: -23,
    conn_2: 20,
    conn_3: 111,
    ojama: -300,
    // U-shape
    non_u_shape: -50,
    non_u_shape_sq: 0,
    // Frames
    frame: -1,
    frame_by_chain: -5,
    frame_by_chigiri: -3,
    // Detected chains
    detected_need: -25,
    detected_keys: -250,
    detected_score_per_k: 50,
};

#[allow(dead_code)]
const HURRY: Evaluator = Evaluator {
    name: "hurry",
    ..Evaluator::zero()
};
const ZENKESHI: Evaluator = Evaluator {
    name: "zenkeshi",
    ..Evaluator::zero()
};
#[allow(dead_code)]
const TSUBUSHI: Evaluator = Evaluator {
    name: "tsubushi",
    ..Evaluator::zero()
};
