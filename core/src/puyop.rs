use crate::{board::Board, color::PuyoColor};

const URL_PREFIX: &'static str = "http://www.puyop.com/s/";
const CHARS: &'static str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ[]";

fn encode_board(board: &Board) -> String {
    fn color_to_int(color: PuyoColor) -> usize {
        match color {
            PuyoColor::EMPTY => 0,
            PuyoColor::RED => 1,
            PuyoColor::GREEN => 2,
            PuyoColor::BLUE => 3,
            PuyoColor::YELLOW => 4,
            PuyoColor::OJAMA => 6,
            _ => unreachable!(),
        }
    }

    let mut encoded = String::new();
    let mut first = true;

    for y in (1..=13).rev() {
        for x in [1, 3, 5] {
            if first && board.is_empty(x, y) && board.is_empty(x + 1, y) {
                continue;
            }

            first = true;
            let puyo_l = color_to_int(board.get(x, y));
            let puyo_r = color_to_int(board.get(x + 1, y));
            let pair = puyo_l << 3 | puyo_r;
            let pair_c = CHARS.chars().nth(pair).unwrap();
            encoded.push(pair_c);
        }
    }

    encoded
}

pub fn construct_board_url(board: &Board) -> String {
    format!("{}{}", URL_PREFIX, encode_board(board))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn construct_board_url_19chain() {
        let board = Board::from(concat!(
            ".G.BRG", // 13
            "GBRRYR", // 12
            "RRYYBY", // 11
            "RGYRBR", // 10
            "YGYRBY", // 9
            "YGBGYR", // 8
            "GRBGYR", // 7
            "BRBYBY", // 6
            "RYYBYY", // 5
            "BRBYBR", // 4
            "BGBYRR", // 3
            "YGBGBG", // 2
            "RBGBGG"  // 1
        ));

        assert_eq!(
            construct_board_url(&board),
            "http://www.puyop.com/s/23aj9x9AsaxpyxsyqxhqxpssczApspqs9yqqbji"
        );
    }
}
