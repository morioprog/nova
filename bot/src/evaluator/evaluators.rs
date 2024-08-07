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
    bump: -16,
    dent: -122,
    dead_cells: -23,
    conn_2_v: 52,
    conn_2_h: 115,
    conn_3: 64,
    ojama: -300,
    // U-shape
    non_u_shape: -4,
    non_u_shape_sq: -10,
    // Frames
    frame: -1,
    frame_by_chain: -5,
    frame_by_chigiri: -3,
    // Detected chains
    detected_need: -6,
    detected_keys: 0,
    detected_score_per_k: 112,
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
