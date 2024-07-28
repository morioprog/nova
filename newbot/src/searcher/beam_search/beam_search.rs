use core::{placement::Placement, player_state::PlayerState};
use std::{sync::mpsc, thread};

use crate::{
    decision::Decision,
    evaluator::Evaluator,
    searcher::{
        beam_search::node::{sort_by_eval, Node},
        Searcher,
    },
};

/// Number of threads available.
const PARALLEL: usize = 20;

pub(crate) struct BeamSearcher;

impl Searcher for BeamSearcher {
    fn search(
        player_state: &PlayerState,
        evaluator: &Evaluator,
        think_frame: Option<u32>,
    ) -> Decision {
        let (depth, width) = get_best_depth_and_width(think_frame);
        let parallel_n = if player_state.tumos.available_tumo_len() < depth {
            PARALLEL
        } else {
            1
        };

        let (decision_sender, decision_receiver) = mpsc::channel();
        for _ in 0..parallel_n {
            let decision_sender = decision_sender.clone();
            let player_state = player_state.clone();
            let evaluator = evaluator.clone();

            thread::spawn(move || {
                decision_sender
                    .send(search_single_thread(depth, width, player_state, evaluator))
                    .ok();
            });
        }
        let decisions: Vec<Decision> = decision_receiver.iter().take(parallel_n).collect();

        let mut scores = [[0; 4]; 7];
        for decision in &decisions {
            let placement = &decision.placements[0];
            scores[placement.axis_x()][placement.rot()] += 1;
        }

        let best_placement = Placement::placements_non_zoro()
            .iter()
            .max_by(|p1, p2| scores[p1.axis_x()][p1.rot()].cmp(&scores[p2.axis_x()][p2.rot()]))
            .unwrap();
        decisions
            .into_iter()
            .find(|decision| &decision.placements[0] == best_placement)
            .unwrap()
    }
}

fn get_best_depth_and_width(think_frame: Option<u32>) -> (usize, usize) {
    if let Some(frame) = think_frame {
        if frame <= 2 {
            (20, 20)
        } else if frame <= 8 {
            (30, 60)
        } else {
            (40, 140)
        }
    } else {
        (30, 60)
    }
}

fn search_single_thread(
    depth: usize,
    width: usize,
    mut player_state: PlayerState,
    evaluator: Evaluator,
) -> Decision {
    // monte carlo
    let visible = player_state.tumos.available_tumo_len();
    player_state.tumos.extend_randoms(depth - visible);

    let mut nodes = vec![Node::from_player_state(&player_state, &[], &evaluator)];
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
        nxt_nodes.resize(width, Node::default());
        nodes = nxt_nodes.clone();
    }

    Decision {
        placements: nodes[0].placements.clone(),
        chain: nodes[0].chain.clone(),
        logging: Some(format!(
            "eval: {:>6}\ntactics: {:>7}\na",
            nodes[0].eval_score, evaluator.name
        )),
    }
}
