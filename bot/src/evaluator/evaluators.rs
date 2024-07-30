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
    bump: -50,
    dent: -50,
    dead_cells: -20,
    conn_2: 50,
    conn_3: 20,
    ojama: -300,
    // U-shape
    non_u_shape: -50,
    non_u_shape_sq: 0,
    // Frames
    frame: -1,
    frame_by_chain: -5,
    frame_by_chigiri: -3,
    // Detected chains
    score_per_k: 100,
};

const HURRY: Evaluator = Evaluator {
    name: "hurry",
    ..Evaluator::zero()
};
const ZENKESHI: Evaluator = Evaluator {
    name: "zenkeshi",
    ..Evaluator::zero()
};
const TSUBUSHI: Evaluator = Evaluator {
    name: "tsubushi",
    ..Evaluator::zero()
};
