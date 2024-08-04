use core::{chain::Chain, placement::Placement, player_state::PlayerState};

use crate::{
    decision::Decision,
    evaluator::Evaluator,
    searcher::{
        beam_search::node::{sort_by_eval, Node},
        Searcher,
    },
};

pub struct BeamSearcher;

impl Searcher for BeamSearcher {
    fn search(
        player_state: &PlayerState,
        evaluator: &Evaluator,
        think_frame: Option<u32>,
    ) -> Decision {
        let (depth, width) = get_best_depth_and_width(think_frame);
        let depth = depth.min(player_state.tumos.available_tumo_len());

        let mut nodes = vec![Node::from_player_state(player_state, &[], evaluator)];
        let mut nxt_nodes = Vec::<Node>::with_capacity(width * 2);
        let mut nxt_sorted;

        for d in 0..depth {
            let tumo = &player_state.tumos[d];
            let placements_itr = if tumo.is_zoro() {
                Placement::placements_zoro().iter()
            } else {
                Placement::placements_non_zoro().iter()
            };

            nxt_nodes.clear();
            nxt_sorted = false;

            for placement in placements_itr {
                for node in &nodes {
                    if !node.player_state.board.is_placeable(placement) {
                        continue;
                    }

                    let nxt = node.place_tumo(tumo, placement, &evaluator);
                    if node.player_state.board.is_dead() {
                        continue;
                    }

                    if nxt_sorted && nxt_nodes[width - 1].eval_score > nxt.eval_score {
                        continue;
                    }
                    nxt_nodes.push(nxt);

                    if nxt_nodes.len() >= width * 2 {
                        sort_by_eval(&mut nxt_nodes);
                        nxt_nodes.resize(width, Node::default());
                        nxt_sorted = true;
                    }
                }
            }

            if nxt_nodes.is_empty() {
                break;
            }

            sort_by_eval(&mut nxt_nodes);
            if nxt_nodes.len() > width {
                nxt_nodes.resize(width, Node::default());
            }
            nodes = nxt_nodes.clone();
        }

        if nodes.is_empty() || nodes[0].placements.is_empty() {
            return Decision {
                placements: vec![Placement::new(3, 0)],
                chain: Chain::default(),
                logging: Some("muri...".to_owned()),
            };
        }

        Decision {
            placements: nodes[0].placements.clone(),
            chain: nodes[0].chain.clone(),
            logging: Some(format!(
                "eval: {:>6}\ntactics: {:>7}",
                nodes[0].eval_score, evaluator.name
            )),
        }
    }
}

fn get_best_depth_and_width(think_frame: Option<u32>) -> (usize, usize) {
    if let Some(frame) = think_frame {
        // TODO: for debug mode, super ad-hoc!!
        if frame >= 1000000 {
            (frame as usize % 1000, (frame as usize / 1000) % 1000)
        } else if frame >= 24 {
            (3, 121)
        } else if frame >= 8 {
            (3, 22)
        } else {
            (2, 22)
        }
    } else {
        // TODO: impl simple DFS instead
        (2, 22)
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
            Tumos::new(&vec![Tumo::new(RED, GREEN)]),
            Tumos::new(&vec![Tumo::new_zoro(BLUE)]),
        ];

        for board in &boards {
            for tumos in &tumos_pattern {
                let player_state = PlayerState::new(board.clone(), tumos.clone(), 0, 0, 0, 0, 0, 0);
                let decision = BeamSearcher::search(&player_state, &BUILD, None);

                assert!(!decision.placements.is_empty());
                assert!(board.is_placeable(decision.placements.first().unwrap()));
            }
        }
    }
}
