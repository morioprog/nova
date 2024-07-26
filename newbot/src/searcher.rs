mod beam_search;
mod random;
// mod dfs;

use core::player_state::PlayerState;

pub(crate) use beam_search::BeamSearcher;
pub(crate) use random::RandomSearcher;

use crate::{decision::Decision, evaluator::Evaluator};

pub(crate) trait Searcher {
    /// Returns (the best decision, list of chains that could be fired)
    fn search(
        player_state: &PlayerState,
        evaluator: &Evaluator,
        think_frame: Option<u32>,
    ) -> Decision;
}
