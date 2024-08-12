use core::{board::WIDTH, player_state::PlayerState};

use super::Evaluator;

pub fn select_best_evaluator(
    player_state_1p: &PlayerState,
    player_state_2p: Option<&PlayerState>,
) -> Evaluator {
    let player_state_2p = match player_state_2p {
        Some(state) => state,
        None => return select_best_build_evaluator(player_state_1p),
    };

    if player_state_1p.carry_over >= 70 * 30 && player_state_2p.carry_over >= 70 * 30 {
        return ZENKESHI;
    }

    select_best_build_evaluator(player_state_1p)
}

pub fn select_best_build_evaluator(player_state: &PlayerState) -> Evaluator {
    let puyo_count = player_state.board.height_array()[1..=WIDTH]
        .iter()
        .sum::<usize>();

    // TODO: we may want to use different evaluator if there's many ojamas?
    if puyo_count >= 9 * WIDTH {
        BUILD_ENDGAME
    } else if puyo_count >= 5 * WIDTH {
        BUILD_MIDGAME
    } else {
        BUILD
    }
}

pub const BUILD: Evaluator = Evaluator {
    name: "build",
    bump: -34,
    dent: -119,
    dead_cells: -20,
    conn_2_v: 37,
    conn_2_h: 156,
    ojama: -300,
    // U-shape
    non_u_shape: -4,
    non_u_shape_sq: -9,
    // Frames
    frame: -1,
    frame_by_chain: -5,
    frame_by_chigiri: -3,
    // Detected chains
    detected_need: -6,
    detected_keys: 0,
    detected_score_per_k: 112,
};

pub const BUILD_MIDGAME: Evaluator = Evaluator {
    name: "build_mid",
    bump: -70,
    dent: -265,
    non_u_shape_sq: -53,
    conn_2_v: 110,
    conn_2_h: 84,
    detected_need: -32,
    detected_keys: 0,
    detected_score_per_k: 173,
    ..BUILD
};

pub const BUILD_ENDGAME: Evaluator = Evaluator {
    name: "build_end",
    bump: -70,
    dent: -265,
    non_u_shape_sq: -53,
    conn_2_v: 110,
    conn_2_h: 84,
    detected_need: -80,
    detected_keys: 0,
    detected_score_per_k: 199,
    ..BUILD_MIDGAME
};

#[allow(dead_code)]
pub const HURRY: Evaluator = Evaluator {
    name: "hurry",
    ..Evaluator::zero()
};
pub const ZENKESHI: Evaluator = Evaluator {
    name: "zenkeshi",
    ..Evaluator::zero()
};
#[allow(dead_code)]
pub const TSUBUSHI: Evaluator = Evaluator {
    name: "tsubushi",
    ..Evaluator::zero()
};
