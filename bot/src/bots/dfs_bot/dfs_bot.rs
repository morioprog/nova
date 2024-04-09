use core::{placement::Placement, player_state::PlayerState};

use eval::{Evaluator, NORMAL_EVALUATOR};

use super::node::Node;
use crate::{decision::DecisionWithoutElapsed, Bot};

pub struct DfsBot {
    pub evaluator: Evaluator,
}

impl Bot for DfsBot {
    fn name(&self) -> &'static str {
        "DfsBot"
    }

    fn think_internal_1p(&self, player_state: &PlayerState) -> DecisionWithoutElapsed {
        // cannot read more than depth 2
        let depth = player_state.tumos.len().min(2);

        let mut nodes: Vec<Node> =
            vec![Node::from_player_state(player_state, &[], NORMAL_EVALUATOR)];
        let mut chain_nodes = vec![];

        for d in 0..depth {
            let tumo = player_state.tumos[d];
            let placements_itr = if tumo.is_zoro() {
                Placement::placements_zoro().iter()
            } else {
                Placement::placements_non_zoro().iter()
            };

            let mut nxt_nodes = Vec::with_capacity(nodes.len() * placements_itr.len());

            for placement in placements_itr {
                for node in &nodes {
                    if !node.player_state.board.is_placeable(placement) {
                        continue;
                    }

                    let nxt = node.place_tumo(&tumo, placement, NORMAL_EVALUATOR);
                    if nxt.chain.is_some() {
                        chain_nodes.push(nxt);
                    } else {
                        nxt_nodes.push(nxt);
                    }
                }
            }

            nodes = nxt_nodes;
        }

        let best_chain = chain_nodes.iter().max_by(|x, y| {
            let x_score = x.chain.as_ref().unwrap().score();
            let y_score = y.chain.as_ref().unwrap().score();
            x_score.cmp(&y_score)
        });
        if let Some(chain) = best_chain {
            let score = chain.chain.as_ref().unwrap().score();
            if score >= 70000 || nodes.is_empty() {
                return DecisionWithoutElapsed {
                    placements: chain.placements.clone(),
                    logging: Some(format!("Found chain with score {}", score)),
                };
            }
        }

        let Some(best_node) = nodes.iter().max_by(|x, y| {
            let x_score = x.eval_score.unwrap();
            let y_score = y.eval_score.unwrap();
            x_score.cmp(&y_score)
        }) else {
            unreachable!()
        };

        DecisionWithoutElapsed {
            placements: best_node.placements.clone(),
            logging: Some(format!(
                "Current board score {}",
                best_node.eval_score.unwrap()
            )),
        }
    }

    fn think_internal_2p(
        &self,
        _player_state_1p: &PlayerState,
        _player_state_2p: &PlayerState,
    ) -> DecisionWithoutElapsed {
        todo!()
    }
}
