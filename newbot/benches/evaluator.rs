#![feature(test)]

extern crate test;

use core::{
    board::Board,
    color::PuyoColor::*,
    player_state::PlayerState,
    tumo::{Tumo, Tumos},
};

use nova_newbot::evaluator::BUILD;
use test::Bencher;

// 2,452 ns/iter (+/- 247)
// TODO: measure again once added major features
#[bench]
fn bench_evaluate(b: &mut Bencher) {
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
    let player_state = PlayerState::new(board, tumos, 1, 2, 3, 4, 5, 0);

    b.iter(|| test::black_box(BUILD.clone().evaluate(&player_state.clone())));
}
