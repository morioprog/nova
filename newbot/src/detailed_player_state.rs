use core::player_state::PlayerState;

#[derive(Clone, Default)]
pub struct DetailedPlayerState {
    pub player_state: PlayerState,
    // Frames
    pub frame_since_control_start: u32,
    pub frame_by_chain: u32,
    pub frame_by_chigiri: u32,
}

impl From<PlayerState> for DetailedPlayerState {
    fn from(value: PlayerState) -> Self {
        Self {
            player_state: value,
            frame_since_control_start: 0,
            frame_by_chain: 0,
            frame_by_chigiri: 0,
        }
    }
}

impl std::ops::Deref for DetailedPlayerState {
    type Target = PlayerState;

    fn deref(&self) -> &Self::Target {
        &self.player_state
    }
}

impl std::ops::DerefMut for DetailedPlayerState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.player_state
    }
}
