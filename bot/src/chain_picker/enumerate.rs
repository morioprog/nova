use core::{chain::Chain, placement::Placement, player_state::PlayerState, tumo::Tumo};

use crate::decision::Decision;

const MAX_TUMOS: usize = 3;

const fn max_capacity(max_tumos: usize) -> usize {
    Placement::placements_non_zoro().len().pow(max_tumos as u32)
}

/// Return the list of chains that can be fired within [MAX_TUMOS] tumos from the current [player_state].
pub fn enumerate_fireable_chains(player_state: &PlayerState) -> Vec<Decision> {
    let max_tumos = MAX_TUMOS.min(player_state.tumos.available_tumo_len());

    let mut nodes = vec![Node(player_state.clone(), vec![], Chain::default())];
    let mut nxt_nodes = Vec::<Node>::with_capacity(max_capacity(max_tumos));
    let mut candidate_chains = Vec::<Decision>::with_capacity(max_capacity(max_tumos) * 2);

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
                if node.0.board.is_dead() {
                    continue;
                }

                nxt_nodes.push(nxt_node.clone());
                if fired {
                    candidate_chains.push(nxt_node.into());
                }
            }
        }

        if nxt_nodes.is_empty() {
            break;
        }

        nodes = nxt_nodes.clone();
    }

    if candidate_chains.is_empty() {
        return candidate_chains;
    }

    // sort by frame (asc)
    candidate_chains.sort_by(|a, b| a.chain.frame().cmp(&b.chain.frame()));

    // take only if score is greater than the previous one (using in-memory swapping for performance sake)
    let mut tail = 0;
    for i in 1..candidate_chains.len() {
        let prv_score = candidate_chains[tail].chain.score();
        let cur_score = candidate_chains[i].chain.score();

        if prv_score < cur_score {
            tail += 1;

            // let candidate_chains[tail] be the last one
            if tail != i {
                candidate_chains.swap(tail, i);
            }
        }
    }
    candidate_chains.resize(tail + 1, Decision::default());

    candidate_chains
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

        let new_chain = self.2.clone() + fired.clone() + Chain::new(0, 0, place_frame);

        (
            Self(new_player_state, new_placements, new_chain),
            fired.chain() > 0,
        )
    }
}

impl From<Node> for Decision {
    fn from(value: Node) -> Self {
        Self {
            placements: value.1.clone(),
            chain: value.2.clone(),
            logging: Some(format!("fire: {:>6}", value.2.score())),
        }
    }
}

#[cfg(test)]
mod tests {
    use core::{board::Board, color::PuyoColor::*, tumo::Tumos};

    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_enumerate_fireable_chains() {
        let board = Board::from(concat!(
            "G.....", // 4
            "GG..Y.", // 3
            "RBBGY.", // 2
            "RRBGG.", // 1
        ));
        let tumos = Tumos::new(&[
            Tumo::new(RED, GREEN),
            Tumo::new(BLUE, YELLOW),
            Tumo::new(YELLOW, GREEN),
        ]);
        let player_state = PlayerState::new(board, tumos, 0, 0, 0, 0, 0, 0);

        let decisions = enumerate_fireable_chains(&player_state);

        assert_eq!(decisions.len(), 10);
        // should be sorted by both frame and score
        assert!(is_sorted(decisions.iter().map(|d| d.chain.frame())));
        assert!(is_sorted(decisions.iter().map(|d| d.chain.score())));
        // smallest chain
        let smallest = decisions.first().unwrap();
        assert_eq!(smallest.chain, Chain::new(1, 40, 103));
        assert_eq!(smallest.placements[0], Placement::new(4, 3));
        // biggest chain
        let biggest = decisions.last().unwrap();
        assert_eq!(biggest.chain, Chain::new(5, 4840, 525));
        assert_eq!(biggest.placements[0], Placement::new(3, 0));
        assert_eq!(biggest.placements[1], Placement::new(4, 2));
        assert_eq!(biggest.placements[2], Placement::new(5, 0));

        // > DEBUG
        // for decision in decisions {
        //     let placements = decision.placements;
        //     let chain = decision.chain;
        //     println!(
        //         "chain: {}, score: {}, frame: {}",
        //         chain.chain(),
        //         chain.score(),
        //         chain.frame()
        //     );
        //     for placement in placements {
        //         println!("{}, {}", placement.axis_x(), placement.rot());
        //     }
        //     println!();
        // }
    }

    #[test]
    fn test_enumerate_fireable_chains_empty() {
        let board = Board::from(concat!(
            "RGRGRG", // 2
            "GRGRGR", // 1
        ));
        let tumos = Tumos::new(&[
            Tumo::new(BLUE, YELLOW),
            Tumo::new(BLUE, YELLOW),
            Tumo::new(BLUE, YELLOW),
        ]);
        let player_state = PlayerState::new(board, tumos, 0, 0, 0, 0, 0, 0);

        assert!(enumerate_fireable_chains(&player_state).is_empty());
    }

    fn is_sorted<T: Ord + Clone>(iter: impl Iterator<Item = T>) -> bool {
        iter.tuple_windows().all(|(a, b)| a <= b)
    }
}
