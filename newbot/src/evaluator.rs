use core::{
    board::{Board, HEIGHT, WIDTH},
    chain::Chain,
    player_state::PlayerState,
    search::ComplementedPuyo,
};

pub(crate) use evaluators::{select_best_evaluator, BUILD};
use feature_extraction::BoardFeature;

mod evaluators;
mod feature_extraction;

pub(crate) struct Evaluator {
    pub bump: i32,
    pub dent: i32,
    pub dead_cells: i32,
    pub conn_2: i32,
    pub conn_3: i32,
    // U-shape
    pub non_u_shape: i32,
    pub non_u_shape_sq: i32,
    // Detected chains
    /// Sum of scores of detected chains divided by 1024.
    /// (Using 1024 instead of 1000 (<=> "k") since the division can be done by a simple bit shift.)
    pub score_per_k: i32,
}

impl Evaluator {
    fn evaluate(&self, player_state: &PlayerState) -> i32 {
        debug_assert!(player_state.board.popping_puyos().is_none());

        if player_state.board.is_dead() {
            return i32::MIN;
        }

        let heights = player_state.board.height_array();
        let avg_height = heights[1..=WIDTH].iter().sum::<usize>() / WIDTH;

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

        let (non_u_shape, non_u_shape_sq) = player_state.board.non_u_shape();
        let r_shift = if avg_height <= 3 {
            3
        } else if avg_height >= HEIGHT - 2 {
            1
        } else {
            0
        };
        score += (self.non_u_shape * non_u_shape) >> r_shift;
        score += (self.non_u_shape_sq * non_u_shape_sq) >> r_shift;

        let mut score_per_k = 0;
        player_state.board.detect_potential_chain(
            2,
            |_board: Board, _cp: ComplementedPuyo, chain: Chain| {
                // devide by 1024
                score_per_k += chain.score() >> 10;
            },
        );
        score += self.score_per_k * (score_per_k as i32);

        score
    }

    const fn zero() -> Self {
        Self {
            bump: 0,
            dent: 0,
            dead_cells: 0,
            conn_2: 0,
            conn_3: 0,
            // U-shape
            non_u_shape: 0,
            non_u_shape_sq: 0,
            // Detected chains
            score_per_k: 0,
        }
    }
}
