#![feature(test)]

extern crate test;

use core::{
    board::Board,
    color::PuyoColor::*,
    player_state::PlayerState,
    tumo::{Tumo, Tumos},
};

use nova_bot::{evaluator::BUILD, DetailedPlayerState};
use test::Bencher;

// max_depth == 1:  3,284 ns/iter (+/- 214)
// max_depth == 6: 14,421 ns/iter (+/- 880)
// TODO: measure again once added major features
#[bench]
fn bench_evaluate_1(b: &mut Bencher) {
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
    let player_state: DetailedPlayerState = PlayerState::new(board, tumos, 1, 2, 3, 4, 5, 0).into();

    b.iter(|| test::black_box(BUILD.clone().evaluate(&player_state.clone())));
}

// max_depth = 1:  3,443 ns/iter (+/- 51)
// max_depth = 6: 36,332 ns/iter (+/- 1,265)
#[bench]
fn bench_evaluate_2(b: &mut Bencher) {
    let board = Board::from(concat!(
        "....Y.", // 7
        "....BG", // 6
        "....GG", // 5
        "Y.GBYY", // 4
        "YGGRRR", // 3
        "GYYBYG", // 2
        "BBBYYG", // 1
    ));
    let tumos = Tumos::new(&[
        Tumo::new(RED, BLUE),
        Tumo::new(YELLOW, GREEN),
        Tumo::new(YELLOW, GREEN),
    ]);
    let player_state: DetailedPlayerState = PlayerState::new(board, tumos, 1, 2, 3, 4, 5, 0).into();

    b.iter(|| test::black_box(BUILD.clone().evaluate(&player_state.clone())));
}
