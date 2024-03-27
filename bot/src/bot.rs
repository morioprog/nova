use core::player_state::PlayerState;

use crate::Decision;

pub trait Bot {
    fn new() -> Self;
    fn name(&self) -> &'static str;
    fn think_1p(&self, player_state: PlayerState) -> Decision;
    fn think_2p(&self, player_state_1p: PlayerState, player_state_2p: PlayerState) -> Decision;
}
