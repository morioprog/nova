use core::{board::Board, placement::Placement, player_state::PlayerState, tumo::Tumos};
use std::io::Write;

use log::{debug, info};

const NOVA_NAME: &str = "nova";
const NOVA_AUTHOR: &str = "morioprog";

pub struct Upi;

pub trait UpiMixin {
    fn receive_upi();
    fn send_ids();
    fn send_upiok();

    fn receive_isready();
    fn send_readyok();

    fn receive_upinewgame();

    fn receive_position_or_gameover() -> (Option<(PlayerState, PlayerState)>, Option<bool>);
    fn receive_go() -> u32;
    fn send_bestmove(placement: Placement);
}

impl Upi {
    fn read_stdin() -> String {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        input = input.trim().to_string();

        if input == "quit" {
            info!("received quit, closing app");
            std::process::exit(0);
        }

        input
    }

    fn print_and_flush(str: impl Into<String>) {
        println!("{}", str.into());
        std::io::stdout().flush().unwrap();
    }
}

impl UpiMixin for Upi {
    fn receive_upi() {
        debug!("waiting for `upi`");
        let input = Self::read_stdin();
        assert_eq!(input, "upi", "expected `upi`, received `{}`", input)
    }

    fn send_ids() {
        Self::print_and_flush(format!("id name {}", NOVA_NAME));
        Self::print_and_flush(format!("id author {}", NOVA_AUTHOR));
    }

    fn send_upiok() {
        Self::print_and_flush("upiok");
    }

    fn receive_isready() {
        debug!("waiting for `isready`");
        let input = Self::read_stdin();
        assert_eq!(input, "isready", "expected `isready`, received `{}`", input)
    }

    fn send_readyok() {
        Self::print_and_flush("readyok");
    }

    fn receive_upinewgame() {
        debug!("waiting for `upinewgame`");
        let input = Self::read_stdin();
        assert_eq!(
            input, "upinewgame",
            "expected `upinewgame`, received `{}`",
            input
        )
    }

    fn receive_position_or_gameover() -> (Option<(PlayerState, PlayerState)>, Option<bool>) {
        debug!("waiting for `position` or `gameover`");
        let input = Self::read_stdin();
        let tokens: Vec<&str> = input.split(' ').collect();
        let (cmd, params) = tokens.split_first().expect("received empty string");

        assert!(
            *cmd == "position" || *cmd == "gameover",
            "expected `position` or `gameover`, received {}",
            input
        );

        // handle "gameover"
        if *cmd == "gameover" {
            assert_eq!(
                params.len(),
                1,
                "`gameover` should have only one parameter (actual: {})",
                params.len()
            );

            return match params[0] {
                "win" => (None, Some(true)),
                "lose" => (None, Some(false)),
                _ => panic!("`gameover` should provide win or lose, got {}", params[0]),
            };
        }

        // handle "position"
        assert_eq!(
            params.len(),
            14,
            "`position` should have 14 parameters (actual: {})",
            params.len()
        );

        let board_1p: Board = Board::from_pfen(params[0]);
        let tumos_1p: Tumos = Tumos::from(params[1]);
        let frame_1p: u32 = params[2].parse().expect("frame_1p parse failed");
        let carry_over_1p: u32 = params[3].parse().expect("carry_over_1p parse failed");
        let ojama_fixed_1p: u32 = params[4].parse().expect("ojama_fixed_1p parse failed");
        let ojama_incoming_1p: u32 = params[5].parse().expect("ojama_incoming_1p parse failed");
        let current_chain_1p: u32 = params[6].parse().expect("current_chain_1p parse failed");
        let board_2p: Board = Board::from_pfen(params[7]);
        let tumos_2p: Tumos = Tumos::from(params[8]);
        let frame_2p: u32 = params[9].parse().expect("frame_2p parse failed");
        let carry_over_2p: u32 = params[10].parse().expect("carry_over_2p parse failed");
        let ojama_fixed_2p: u32 = params[11].parse().expect("ojama_fixed_2p parse failed");
        let ojama_incoming_2p: u32 = params[12].parse().expect("ojama_incoming_2p parse failed");
        let current_chain_2p: u32 = params[13].parse().expect("current_chain_2p parse failed");

        let player_state_1p = PlayerState::new(
            board_1p,
            tumos_1p,
            frame_1p,
            0,
            carry_over_1p,
            ojama_fixed_1p,
            ojama_incoming_1p,
            current_chain_1p,
        );
        let player_state_2p = PlayerState::new(
            board_2p,
            tumos_2p,
            frame_2p,
            0,
            carry_over_2p,
            ojama_fixed_2p,
            ojama_incoming_2p,
            current_chain_2p,
        );

        (Some((player_state_1p, player_state_2p)), None)
    }

    fn receive_go() -> u32 {
        debug!("waiting for `go`");
        let input = Self::read_stdin();
        let tokens: Vec<&str> = input.split(' ').collect();
        let (cmd, params) = tokens.split_first().expect("received empty string");
        assert!(*cmd == "go", "expected `go`, received {}", input);
        assert_eq!(
            params.len(),
            1,
            "`go` should have only one parameter (actual: {})",
            params.len()
        );

        params[0].parse().expect("ms parse failed")
    }

    fn send_bestmove(placement: Placement) {
        Self::print_and_flush(format!(
            "bestmove {} {}",
            placement.axis_x(),
            placement.rot()
        ));
    }
}
