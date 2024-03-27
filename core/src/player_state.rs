use crate::{board::Board, tumo::Tumos};

pub struct PlayerState {
    pub board: Board,
    pub tumos: Tumos,
    pub frame: u32,
    pub score: u32,
    pub carry_over: u32,
    pub ojama_fixed: u32,
    pub ojama_incoming: u32,
    pub current_chain: u32,
}

impl PlayerState {
    pub fn initial_state(tumos: Tumos) -> Self {
        Self {
            tumos,
            ..Self::zero()
        }
    }

    pub fn zero() -> Self {
        Self::new(Board::new(), Tumos::default(), 0, 0, 0, 0, 0, 0)
    }

    pub const fn new(
        board: Board,
        tumos: Tumos,
        frame: u32,
        score: u32,
        carry_over: u32,
        ojama_fixed: u32,
        ojama_incoming: u32,
        current_chain: u32,
    ) -> Self {
        Self {
            board,
            tumos,
            frame,
            score,
            carry_over,
            ojama_fixed,
            ojama_incoming,
            current_chain,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        color::PuyoColor::{BLUE, GREEN, RED},
        tumo::Tumo,
    };

    #[test]
    fn initial_state() {
        let mut tumos = Tumos::default();
        tumos.push(&Tumo::new_zoro(RED));
        tumos.push(&Tumo::new(BLUE, GREEN));
        tumos.push(&Tumo::new(BLUE, RED));

        let player_state = PlayerState::initial_state(tumos.clone());

        assert_eq!(player_state.tumos.len(), tumos.len());
        assert_eq!(player_state.tumos[0], tumos[0]);
        assert_eq!(player_state.tumos[1], tumos[1]);
        assert_eq!(player_state.tumos[2], tumos[2]);
        assert_eq!(player_state.board, Board::new());
    }
}
