use core::player_state::PlayerState;

use super::ChainPicker;
use crate::decision::Decision;

struct Houwa;

impl ChainPicker for Houwa {
    fn pick_chain(
        _player_state_1p: &PlayerState,
        _player_state_2p: &PlayerState,
        chains: &[Decision],
    ) -> Option<Decision> {
        // TODO: refine
        chains
            .iter()
            .max_by(|a, b| a.chain.score().cmp(&b.chain.score()))
            .cloned()
            .filter(|d| d.chain.score() >= 50000)
    }
}
