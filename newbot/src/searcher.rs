mod random;
// mod dfs;
// mod beam_search;

use core::player_state::PlayerState;

use crate::decision::Decision;

trait Searcher {
    /// Returns (the best decision, list of chains that could be fired)
    fn search(&self, player_state: PlayerState) -> (Decision, Vec<Decision>);
}
