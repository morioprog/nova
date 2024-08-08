use bot::Nova;
use nova_simulator::simulate_1p;

const SIMULATE_N: usize = 1000;

// As of Aug 7th 2024
// >=  80k: 76.8%
// >=  90k: 36.0%
// >= 100k: 13.4%
// TODO: Specify (6, 22) as depth/width
fn main() {
    let mut score_cnt = [0; 20];
    let mut think_ms_avg = 0.0;

    for sim in 1..=SIMULATE_N {
        let simulate_result = simulate_1p(Nova::default(), None, Some(2));
        think_ms_avg += simulate_result
            .decisions
            .iter()
            .map(|decision| decision.elapsed.as_millis() as f64)
            .sum::<f64>()
            / simulate_result.decisions.len() as f64;

        for i in 0..score_cnt.len() {
            if simulate_result.score as usize >= i * 10000 {
                score_cnt[i] += 1;
            }
        }

        if sim % (SIMULATE_N / 10) == 0 {
            println!("simulate {:4} done", sim);
        }
    }

    think_ms_avg /= SIMULATE_N as f64;

    println!("batch simulate result:");
    println!("> think: {}", think_ms_avg);
    println!("> scores:");
    for (i, j) in score_cnt.iter().enumerate() {
        println!("  >= {:6}: {}", i * 10000, j);
    }
}
