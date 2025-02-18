use std::io::Write;

use bot::Nova;
use log::info;
use nova_upi::upi::{Upi, UpiMixin};

fn main() {
    setup_logger();
    info!("nova-upi launched");

    Upi::receive_upi();
    Upi::send_ids();
    Upi::send_upiok();

    Upi::receive_isready();
    let nova = Nova::default();
    Upi::send_readyok();

    loop {
        Upi::receive_upinewgame();
        loop {
            let (player_states, is_gameover) = Upi::receive_position_or_gameover();
            if let Some(win) = is_gameover {
                info!("gameover (win?: {})", win);
                break;
            }
            let (player_state_1p, player_state_2p) =
                player_states.expect("player_states should be present");

            let think_ms = Upi::receive_go();
            let think_frame = think_ms * 60 / 1000; // 60 frame/sec

            let decision = nova.think(&player_state_1p, Some(&player_state_2p), Some(think_frame));
            let placement = decision.placements.first().unwrap();
            Upi::send_bestmove(*placement);
        }
    }
}

fn setup_logger() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            let ts = buf.timestamp();
            writeln!(
                buf,
                "[{}] {:<15}#L{:<3} | {}: {}",
                ts,
                record.target(),
                record.line().unwrap_or(0),
                record.level(),
                record.args(),
            )
        })
        .init();
}
