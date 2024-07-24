use crate::{board::Board, color::PuyoColor, placement::Placement, tumo::Tumos};

const URL_PREFIX: &str = "http://www.puyop.com/s/";
const CHARS: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ[]";

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

fn encode_decisions(tumos: &Tumos, placements: &[Placement]) -> String {
    fn color_to_int(color: PuyoColor) -> usize {
        match color {
            PuyoColor::RED => 0,
            PuyoColor::GREEN => 1,
            PuyoColor::BLUE => 2,
            PuyoColor::YELLOW => 3,
            _ => unreachable!(),
        }
    }

    fn placement_to_int(placement: &Placement) -> usize {
        placement.axis_x() << 2 | placement.rot()
    }

    let mut encoded = String::new();

    for (i, placement) in placements.iter().enumerate() {
        let puyo_a = color_to_int(tumos.get_raw(i).unwrap().axis());
        let puyo_c = color_to_int(tumos.get_raw(i).unwrap().child());
        let pair = puyo_a * 5 + puyo_c;
        let plcm = placement_to_int(placement);
        let data = plcm << 7 | pair;
        let data_1 = data & 0b111111;
        let data_2 = (data >> 6) & 0b111111;
        encoded.push(CHARS.chars().nth(data_1).unwrap());
        encoded.push(CHARS.chars().nth(data_2).unwrap());
    }

    encoded
}

pub fn construct_board_url(board: &Board) -> String {
    format!("{}{}", URL_PREFIX, encode_board(board))
}

pub fn construct_sim1p_url(board: &Board, tumos: &Tumos, placements: &[Placement]) -> String {
    format!(
        "{}_{}",
        construct_board_url(board),
        encode_decisions(tumos, placements)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color::PuyoColor::*, tumo::Tumo};

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

    #[test]
    fn construct_sim1p_url_from_empty_board() {
        let tumos = Tumos::new(&[
            Tumo::new(RED, BLUE),
            Tumo::new(YELLOW, YELLOW),
            Tumo::new(GREEN, YELLOW),
            Tumo::new(YELLOW, BLUE),
        ]);
        let placements = &[
            Placement::new(1, 2),
            Placement::new(2, 1),
            Placement::new(4, 2),
            Placement::new(3, 1),
        ];

        assert_eq!(
            construct_sim1p_url(&Board::new(), &tumos, placements),
            "http://www.puyop.com/s/_2cii8Ahq"
        );
    }

    #[test]
    fn construct_sim1p_url_from_nonempty_board() {
        let board = Board::from(concat!(
            "R..RGG", // 2
            "YGGYYB", // 1
        ));
        let tumos = Tumos::new(&[
            Tumo::new(RED, BLUE),
            Tumo::new(YELLOW, YELLOW),
            Tumo::new(GREEN, YELLOW),
            Tumo::new(YELLOW, BLUE),
        ]);
        let placements = &[
            Placement::new(2, 3),
            Placement::new(1, 2),
            Placement::new(3, 0),
            Placement::new(6, 2),
        ];

        assert_eq!(
            construct_sim1p_url(&board, &tumos, placements),
            "http://www.puyop.com/s/81iykz_2mic8ohQ"
        );
    }
}
