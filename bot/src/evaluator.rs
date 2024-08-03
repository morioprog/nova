use core::{
    board::{Board, WIDTH},
    chain::Chain,
    search::ComplementedPuyo,
};

pub(crate) use evaluators::select_best_evaluator;
pub use evaluators::BUILD;
use feature_extraction::BoardFeature;

use crate::DetailedPlayerState;

mod evaluators;
mod feature_extraction;

#[derive(Clone, Copy, Debug)]
pub struct Evaluator {
    pub name: &'static str,
    pub bump: i32,
    pub dent: i32,
    pub dead_cells: i32,
    pub conn_2: i32,
    pub conn_3: i32,
    pub ojama: i32,
    // U-shape
    pub non_u_shape: i32,
    pub non_u_shape_sq: i32,
    // Frames
    pub frame: i32,
    pub frame_by_chain: i32,
    pub frame_by_chigiri: i32,
    // Detected chains
    pub detected_need: i32,
    pub detected_keys: i32,
    /// Sum of scores of detected chains divided by 1024.
    /// (Using 1024 instead of 1000 (<=> "k") since the division can be done by a simple bit shift.)
    pub detected_score_per_k: i32,
}

impl Evaluator {
    pub fn evaluate(&self, player_state: &DetailedPlayerState) -> i32 {
        debug_assert!(player_state.board.popping_puyos().is_none());

        if player_state.board.is_dead() {
            return i32::MIN;
        }

        let mut score = 0i32;

        for x in 1..=WIDTH {
            let bump = player_state.board.bump(x);
            let dent = player_state.board.dent(x);
            score += self.bump * (bump * bump);
            score += self.dent * (dent * dent);
        }

        score += self.dead_cells * player_state.board.dead_cells();

        let (conn_2, conn_3) = player_state.board.connectivity();
        score += self.conn_2 * conn_2;
        score += self.conn_3 * conn_3;

        score += self.ojama * player_state.board.ojama_count();

        let (non_u_shape, non_u_shape_sq) = player_state.board.non_u_shape();
        score += self.non_u_shape * non_u_shape;
        score += self.non_u_shape_sq * non_u_shape_sq;

        score += self.frame * player_state.frame_since_control_start as i32;
        score += self.frame_by_chain * player_state.frame_by_chain as i32;
        score += self.frame_by_chigiri * player_state.frame_by_chigiri as i32;

        let mut detected_score = i32::MIN;
        player_state.board.detect_potential_chain(
            3,
            6,
            |_board: Board, fire_x: usize, cp: ComplementedPuyo, chain: Chain| {
                let mut detected_score_tmp = 0;

                let need = cp.get(fire_x) as i32;
                detected_score_tmp += self.detected_need * need;
                let keys = cp.sum() as i32 - need;
                detected_score_tmp += self.detected_keys * keys;

                // devide by 1024
                detected_score_tmp += self.detected_score_per_k * (chain.score() >> 10) as i32;

                detected_score = detected_score.max(detected_score_tmp);
            },
        );
        if detected_score > i32::MIN {
            score += detected_score;
        }

        score
    }

    const fn zero() -> Self {
        Self {
            name: "noname",
            bump: 0,
            dent: 0,
            dead_cells: 0,
            conn_2: 0,
            conn_3: 0,
            ojama: 0,
            // U-shape
            non_u_shape: 0,
            non_u_shape_sq: 0,
            // Frames
            frame: 0,
            frame_by_chain: 0,
            frame_by_chigiri: 0,
            // Detected chains
            detected_need: 0,
            detected_keys: 0,
            detected_score_per_k: 0,
        }
    }
}
