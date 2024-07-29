use core::tumo::Tumos;
use std::{
    sync::{Arc, Mutex},
    thread,
};

use bot::{evaluator::Evaluator, Nova};
use simulator::simulate_1p;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
struct SimulateResult {
    /// Number of rounds that a bot successfully fired a huge chain.
    chain_success: u32,
    /// Sum of scores.
    score: u32,
    /// Sum of round lengths.
    tumos: u32,
}

impl std::ops::Add for SimulateResult {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            chain_success: self.chain_success + rhs.chain_success,
            score: self.score + rhs.score,
            tumos: self.tumos + rhs.tumos,
        }
    }
}

impl PartialOrd for SimulateResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // chain_success: the greater the better
        match self.chain_success.partial_cmp(&other.chain_success)? {
            std::cmp::Ordering::Equal => (/* continue */),
            other_result => return Some(other_result),
        }
        // score: the greater the better
        match self.score.partial_cmp(&other.score)? {
            std::cmp::Ordering::Equal => (/* continue */),
            other_result => return Some(other_result),
        }
        // tumos: the lesser the better
        Some(self.tumos.partial_cmp(&other.tumos)?.reverse())
    }
}

impl Ord for SimulateResult {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub fn select_best_evaluator(evaluators: Vec<Evaluator>) -> Evaluator {
    let n = evaluators.len();
    let simulate_results = Arc::new(Mutex::new(vec![SimulateResult::default(); n]));

    let mut handles = vec![];
    // TODO: pass 20 as parameter (threads)
    for _ in 0..20 {
        let all_v = Arc::clone(&simulate_results);
        let evaluators = evaluators.clone();

        handles.push(thread::spawn(move || {
            let mut sim_v = vec![SimulateResult::default(); n];

            // TODO: pass 10 as parameter (number of tumo patterns)
            for _ in 0..30 {
                let tumos = Tumos::new_random();
                for (i, evaluator) in evaluators.iter().enumerate() {
                    let result = simulate_1p(Nova::with_evaluator(*evaluator), Some(tumos.clone()));
                    sim_v[i] = sim_v[i]
                        + SimulateResult {
                            // TODO: pass 70000 as parameter
                            chain_success: if result.score >= 70000 { 1 } else { 0 },
                            score: result.score,
                            tumos: result.decisions.len() as u32,
                        }
                }
            }

            let mut lock = all_v.lock().unwrap();
            for i in 0..n {
                (*lock)[i] = (*lock)[i] + sim_v[i];
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *evaluators
        .iter()
        .enumerate()
        .max_by_key(|(index, _)| (*simulate_results.lock().unwrap()).get(*index).cloned())
        .unwrap()
        .1
}
