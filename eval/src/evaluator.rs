use core::{
    board::{Board, WIDTH},
    chain::Chain,
    player_state::PlayerState,
    search::ComplementedPuyo,
};

use crate::feature_extraction::BoardFeature;

pub struct Evaluator {
    pub bump: i32,
    pub dent: i32,
    pub dead_cells: i32,
    pub conn_2: i32,
    pub conn_3: i32,
    // Detected chains
    /// Sum of scores of detected chains divided by 1024.
    /// (Using 1024 instead of 1000 (<=> "k") since the division can be done by a simple bit shift.)
    pub score_per_k: i32,
}

pub const NORMAL_EVALUATOR: &Evaluator = &Evaluator {
    bump: -84,
    dent: -352,
    dead_cells: -339,
    conn_2: 52,
    conn_3: 345,
    // Detected chains
    score_per_k: 20,
};

impl Evaluator {
    pub fn evaluate(&self, player_state: &PlayerState) -> i32 {
        debug_assert!(player_state.board.popping_puyos().is_none());

        if player_state.board.is_dead() {
            return i32::MIN;
        }

        let mut score = 0i32;

        for x in 1..=WIDTH {
            score += self.bump * player_state.board.bump(x);
            score += self.dent * player_state.board.dent(x);
        }

        score += self.dead_cells * player_state.board.dead_cells();

        let (conn_2, conn_3) = player_state.board.connectivity();
        score += self.conn_2 * conn_2;
        score += self.conn_3 * conn_3;

        let mut score_per_k = 0;
        player_state.board.detect_potential_chain(
            3,
            |_board: Board, _cp: ComplementedPuyo, chain: Chain| {
                // devide by 1024
                score_per_k += chain.score() >> 10;
            },
        );
        score += self.score_per_k * (score_per_k as i32);

        score
    }
}
