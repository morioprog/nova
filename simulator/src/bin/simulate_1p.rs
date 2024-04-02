use bot::bots::get_bot;
use ghoti_simulator::simulate_1p;

fn main() {
    let bot_name = "RandomBot";
    let bot = get_bot(bot_name);

    let simulate_result = simulate_1p(bot);

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
