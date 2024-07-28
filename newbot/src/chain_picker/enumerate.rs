use core::{chain::Chain, placement::Placement, player_state::PlayerState, tumo::Tumo};

use crate::decision::Decision;

const MAX_TUMOS: usize = 3;

const fn max_capacity(max_tumos: usize) -> usize {
    Placement::placements_non_zoro().len().pow(max_tumos as u32)
}

/// Return the list of chains that can be fired within [MAX_TUMOS] tumos from the current [player_state].
pub(crate) fn enumerate_fireable_chains(player_state: &PlayerState) -> Vec<Decision> {
    let max_tumos = MAX_TUMOS.min(player_state.tumos.available_tumo_len());

    let mut nodes = vec![Node(player_state.clone(), vec![], Chain::default())];
    let mut nxt_nodes = Vec::<Node>::with_capacity(max_capacity(max_tumos));
    let mut candidate_chains = Vec::<DecisionWithFrame>::with_capacity(max_capacity(max_tumos) * 2);

    for d in 0..max_tumos {
        let tumo = player_state.tumos[d];
        let placements_itr = if tumo.is_zoro() {
            Placement::placements_zoro().iter()
        } else {
            Placement::placements_non_zoro().iter()
        };

        nxt_nodes.clear();
        for placement in placements_itr {
            for node in &nodes {
                if !node.0.board.is_placeable(placement) {
                    continue;
                }

                let (nxt_node, fired) = node.place_tumo(&tumo, placement);
                if fired {
                    candidate_chains.push(DecisionWithFrame::from_node(&nxt_node));
                }
                nxt_nodes.push(nxt_node);
            }
        }

        if nxt_nodes.is_empty() {
            break;
        }

        nodes = nxt_nodes.clone();
    }

    // sort by frame (asc)
    candidate_chains.sort_by(|a, b| a.1.cmp(&b.1));

    // take only if score is greater than the previous one (using in-memory swapping for performance sake)
    let mut tail = 0;
    for i in 1..candidate_chains.len() {
        let prv_score = candidate_chains[tail].0.chain.score();
        let cur_score = candidate_chains[i].0.chain.score();

        if prv_score < cur_score {
            tail += 1;

            // let candidate_chains[tail] be the last one
            if tail != i {
                candidate_chains.swap(tail, i);
            }
        }
    }
    candidate_chains.resize(tail + 1, DecisionWithFrame::default());

    candidate_chains.into_iter().map(|dwf| dwf.0).collect()
}

#[derive(Clone)]
struct Node(PlayerState, Vec<Placement>, Chain);
impl Node {
    /// Returns (Node after placing Tumo, true if fired something)
    fn place_tumo(&self, tumo: &Tumo, placement: &Placement) -> (Self, bool) {
        let mut new_player_state = self.0.clone();
        let place_frame = new_player_state.board.place_tumo(tumo, placement).unwrap();
        let fired = new_player_state.board.simulate();
        new_player_state.frame += place_frame + fired.frame();

        let mut new_placements = self.1.clone();
        new_placements.push(*placement);

        let new_chain = self.2.clone() + fired.clone();

        (
            Self(new_player_state, new_placements, new_chain),
            fired.chain() > 0,
        )
    }
}

#[derive(Clone, Default)]
struct DecisionWithFrame(Decision, u32);
impl DecisionWithFrame {
    fn from_node(node: &Node) -> Self {
        Self(
            Decision {
                placements: node.1.clone(),
                chain: node.2.clone(),
                logging: Some(format!("fire: {}", node.2.score())),
            },
            node.0.frame,
        )
    }
}
