use core::{chain::Chain, placement::Placement, player_state::PlayerState};
use std::time::Instant;

use crate::{
    decision::Decision,
    evaluator::Evaluator,
    searcher::{
        beam_search::node::{sort_by_eval, Node},
        Searcher,
    },
};

/// First perform BFS up to depth=2, then continue the search until 84% of the think_frame time has passed.
pub struct ChokudaiSearcher;

const MIN_DEPTH: usize = 2;

impl Searcher for ChokudaiSearcher {
    fn search(
        player_state: &PlayerState,
        evaluator: &Evaluator,
        think_frame: Option<u32>,
    ) -> Decision {
        let start = Instant::now();

        let mut nodes = vec![Node::from_player_state(player_state, &[], evaluator)];
        let mut nxt_nodes = Vec::<Node>::with_capacity(22 * 22);

        for d in 0..MIN_DEPTH {
            let tumo = &player_state.tumos[d];
            let placements_itr = if tumo.is_zoro() {
                Placement::placements_zoro().iter()
            } else {
                Placement::placements_non_zoro().iter()
            };

            nxt_nodes.clear();

            for placement in placements_itr {
                for node in &nodes {
                    if !node.player_state.board.is_placeable(placement) {
                        continue;
                    }

                    let nxt = node.place_tumo(tumo, placement, &evaluator);
                    if node.player_state.board.is_dead() {
                        continue;
                    }

                    nxt_nodes.push(nxt);
                }
            }

            if nxt_nodes.is_empty() {
                break;
            }

            sort_by_eval(&mut nxt_nodes);
            nodes = nxt_nodes.clone();
        }

        if nodes.is_empty() || nodes[0].placements.is_empty() {
            return Decision {
                placements: vec![Placement::new(3, 0)],
                chain: Chain::default(),
                logging: Some("muri...".to_owned()),
            };
        }

        let mut best_node: Option<Node> = None;

        if player_state.tumos.len() >= MIN_DEPTH + 1 {
            let tumo = &player_state.tumos[MIN_DEPTH];
            let placements_itr = if tumo.is_zoro() {
                Placement::placements_zoro().iter()
            } else {
                Placement::placements_non_zoro().iter()
            };

            if let Some(frame) = think_frame {
                let think_frame_ms_84_perc = frame as u128 * (840 / 60);
                for placement in placements_itr.as_ref() {
                    for node in &nodes {
                        if start.elapsed().as_millis() > think_frame_ms_84_perc {
                            break;
                        }

                        if !node.player_state.board.is_placeable(placement) {
                            continue;
                        }

                        let nxt = node.place_tumo(tumo, placement, &evaluator);
                        if nxt.player_state.board.is_dead() {
                            continue;
                        }

                        if let Some(ref best) = best_node {
                            if best.eval_score < nxt.eval_score {
                                best_node = Some(nxt);
                            }
                        } else {
                            best_node = Some(nxt);
                        }
                    }
                }
            }
        }

        let best_node = if let Some(best) = best_node {
            best
        } else {
            nodes[0].clone()
        };

        Decision {
            placements: best_node.placements.clone(),
            chain: best_node.chain.clone(),
            logging: Some(format!(
                "eval: {:>6}\ntactics: {:>7}",
                best_node.eval_score, evaluator.name
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::{
        board::Board,
        color::PuyoColor::*,
        tumo::{Tumo, Tumos},
    };

    use super::*;
    use crate::evaluator::BUILD;

    #[test]
    fn search_returns_valid_placement() {
        let boards: [Board; 8] = [
            [0, 0, 0, 0, 0, 0].into(),
            [11, 11, 11, 11, 11, 11].into(),
            [12, 12, 11, 12, 12, 12].into(),
            [11, 11, 11, 13, 11, 11].into(),
            [11, 13, 11, 11, 11, 11].into(),
            [11, 12, 11, 13, 11, 11].into(),
            [11, 13, 11, 12, 11, 11].into(),
            [11, 13, 11, 13, 11, 11].into(),
        ];
        let tumos_pattern: [Tumos; 2] = [
            Tumos::new(&vec![Tumo::new(RED, GREEN), Tumo::new(BLUE, GREEN)]),
            Tumos::new(&vec![Tumo::new_zoro(BLUE), Tumo::new_zoro(RED)]),
        ];

        for board in &boards {
            for tumos in &tumos_pattern {
                let player_state = PlayerState::new(board.clone(), tumos.clone(), 0, 0, 0, 0, 0, 0);
                let decision = ChokudaiSearcher::search(&player_state, &BUILD, None);

                assert!(!decision.placements.is_empty());
                assert!(board.is_placeable(decision.placements.first().unwrap()));
            }
        }
    }
}
