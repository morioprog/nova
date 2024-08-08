use bot::Nova;
use nova_simulator::simulate_1p;

// TODO: Specify (3, 22) as depth/width
fn main() {
    let nova = Nova::default();
    let simulate_result = simulate_1p(nova, None, Some(2));

    let think_ms_avg = simulate_result
        .decisions
        .iter()
        .map(|decision| decision.elapsed.as_millis() as f64)
        .sum::<f64>()
        / simulate_result.decisions.len() as f64;

    println!("simulate result:");
    println!("> score: {}", simulate_result.score);
    println!("> think: {}", think_ms_avg);
    println!(">   url: {}", simulate_result.create_puyop_url());
}
