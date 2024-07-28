mod enumerate;
pub(crate) mod strategies;

use core::player_state::PlayerState;

pub use enumerate::enumerate_fireable_chains;

use crate::decision::Decision;

pub(crate) trait ChainPicker {
    fn pick_chain(
        player_state_1p: &PlayerState,
        player_state_2p: Option<&PlayerState>,
        chains: &[Decision],
    ) -> Option<Decision>;
}
