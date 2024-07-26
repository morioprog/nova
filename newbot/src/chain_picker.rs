mod houwa;

use core::player_state::PlayerState;

use crate::decision::Decision;

trait ChainPicker {
    fn pick_chain(
        player_state_1p: &PlayerState,
        player_state_2p: &PlayerState,
        chains: &[Decision],
    ) -> Option<Decision>;
}
