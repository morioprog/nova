mod houwa;

use core::player_state::PlayerState;

pub(crate) use houwa::Houwa;

use crate::decision::Decision;

pub(crate) trait ChainPicker {
    fn pick_chain(
        player_state_1p: &PlayerState,
        player_state_2p: Option<&PlayerState>,
        chains: &[Decision],
    ) -> Option<Decision>;
}
