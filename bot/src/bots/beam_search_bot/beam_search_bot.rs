use core::{placement::Placement, player_state::PlayerState};
use std::{sync::mpsc, thread};

use eval::Evaluator;

use super::node::{max_by_chain, Node};
use crate::{bots::beam_search_bot::node::sort_by_eval, decision::DecisionWithoutElapsed, Bot};

/// Number of threads available.
const PARALLEL: usize = 20;

pub struct BeamSearchBot {
    pub evaluator: Evaluator,
}

impl Bot for BeamSearchBot {
    fn name(&self) -> &'static str {
        "BeamSearchBot"
    }

    fn think_internal_1p(
        &self,
        _player_state: &PlayerState,
        _think_frame: Option<u32>,
    ) -> DecisionWithoutElapsed {
        todo!()
    }

    fn think_internal_2p(
        &self,
        player_state_1p: &PlayerState,
        player_state_2p: &PlayerState,
        think_frame: Option<u32>,
    ) -> DecisionWithoutElapsed {
        let (depth, width) = get_best_depth_and_width(think_frame);
        let parallel_n = if player_state_1p.tumos.available_tumo_len() >= depth {
            PARALLEL
        } else {
            1
        };

        let (decision_sender, decision_receiver) = mpsc::channel();
        for _ in 0..parallel_n {
            let decision_sender = decision_sender.clone();
            let player_state_1p = player_state_1p.clone();
            let player_state_2p = player_state_2p.clone();
            let evaluator = self.evaluator.clone();

            thread::spawn(move || {
                decision_sender
                    .send(think_single_thread(
                        depth,
                        width,
                        player_state_1p,
                        player_state_2p,
                        evaluator,
                    ))
                    .ok();
            });
        }
        let decisions: Vec<DecisionWithoutElapsed> =
            decision_receiver.iter().take(parallel_n).collect();

        let mut scores = [[0; 4]; 7];
        for decision in &decisions {
            let placement = &decision.placements[0];
            scores[placement.axis_x()][placement.rot()] += 1;
        }

        let best_placement = Placement::placements_non_zoro()
            .iter()
            .max_by(|d1, d2| scores[d1.axis_x()][d1.rot()].cmp(&scores[d2.axis_x()][d2.rot()]))
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

fn think_single_thread(
    depth: usize,
    width: usize,
    mut player_state_1p: PlayerState,
    _player_state_2p: PlayerState,
    evaluator: Evaluator,
) -> DecisionWithoutElapsed {
    // monte carlo
    let visible = player_state_1p.tumos.available_tumo_len();
    player_state_1p.tumos.extend_randoms(depth - visible);

    let mut nodes: Vec<Node> = vec![Node::from_player_state(&player_state_1p, &[], &evaluator)];
    let mut chain_nodes = vec![];

    for d in 0..depth {
        let tumo = player_state_1p.tumos[d];
        let placements_itr = if tumo.is_zoro() {
            Placement::placements_zoro().iter()
        } else {
            Placement::placements_non_zoro().iter()
        };

        let mut nxt_nodes: Vec<Node> = Vec::with_capacity(width * 2);
        let mut sorted = false;

        for placement in placements_itr {
            for node in &nodes {
                if !node.player_state.board.is_placeable(placement) {
                    continue;
                }

                let nxt: Node = node.place_tumo(&tumo, placement, &evaluator);
                if nxt.chain.is_some() {
                    chain_nodes.push(nxt);
                    continue;
                }

                if sorted && nxt_nodes[width - 1].eval_score > nxt.eval_score {
                    continue;
                }
                nxt_nodes.push(nxt);

                if nxt_nodes.len() >= width * 2 {
                    sort_by_eval(&mut nxt_nodes);
                    nxt_nodes.resize(width, Node::default());
                    sorted = true;
                }
            }
        }

        sort_by_eval(&mut nxt_nodes);
        nodes = nxt_nodes;
    }

    if !chain_nodes.is_empty() {
        let chain_node = max_by_chain(&chain_nodes);
        // TODO: better fire condition
        if chain_node.chain.as_ref().unwrap().score() >= 50000 {
            return DecisionWithoutElapsed {
                placements: chain_node.placements.clone(),
                logging: Some(format!(
                    "chain node: {}",
                    chain_node.chain.as_ref().unwrap().score()
                )),
            };
        }
    }

    DecisionWithoutElapsed {
        placements: nodes[0].placements.clone(),
        logging: Some(format!("eval node: {}", nodes[0].eval_score.unwrap())),
    }
}
