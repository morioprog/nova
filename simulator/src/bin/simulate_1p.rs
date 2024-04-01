use bot::bots::get_bot;
use ghoti_simulator::simulate_1p;

fn main() {
    let bot_name = "RandomBot";
    let bot = get_bot(bot_name);

    let simulate_result = simulate_1p(bot);

    println!("simulate result:");
    println!("> score: {}", simulate_result.score);
    println!(">   url: {}", simulate_result.create_puyop_url());
}
