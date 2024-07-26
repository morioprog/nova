mod random;
// mod dfs;
// mod beam_search;

use core::player_state::PlayerState;

pub(crate) use random::RandomSearcher;

use crate::{decision::Decision, evaluator::Evaluator};

pub(crate) trait Searcher {
    /// Returns (the best decision, list of chains that could be fired)
    fn search(player_state: &PlayerState, evaluator: &Evaluator) -> (Decision, Vec<Decision>);
}
