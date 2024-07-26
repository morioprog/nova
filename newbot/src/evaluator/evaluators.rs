use core::player_state::PlayerState;

use super::Evaluator;

pub fn select_best_evaluator(
    player_state_1p: &PlayerState,
    player_state_2p: &PlayerState,
) -> Evaluator {
    if player_state_1p.carry_over >= 2100 && player_state_2p.carry_over >= 2100 {
        return ZENKESHI;
    }

    BUILD
}

const BUILD: Evaluator = Evaluator {
    bump: -348,
    dent: -152,
    dead_cells: -407,
    conn_2: 21,
    conn_3: 63,
    // U-shape
    non_u_shape: -53,
    non_u_shape_sq: -64,
    // Detected chains
    score_per_k: 30,
};

const HURRY: Evaluator = Evaluator::zero();
const ZENKESHI: Evaluator = Evaluator::zero();
const TUBUSHI: Evaluator = Evaluator::zero();
