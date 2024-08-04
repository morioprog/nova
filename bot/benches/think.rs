#![feature(test)]

extern crate test;

use core::{
    board::Board,
    color::PuyoColor::*,
    player_state::PlayerState,
    tumo::{Tumo, Tumos},
};

use nova_bot::{DetailedPlayerState, Nova};
use test::Bencher;

#[bench]
fn bench_think_1(b: &mut Bencher) {
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

    b.iter(|| test::black_box(Nova::default().think(&player_state.clone(), None, None)));
}

// (2,  22):  28,357,450 ns/iter (+/-   780,026)
// (3,  88): 119,264,260 ns/iter (+/- 2,351,079)
// (3, 306): 343,072,440 ns/iter (+/- 3,420,919)
#[bench]
fn bench_think_2(b: &mut Bencher) {
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

    b.iter(|| test::black_box(Nova::default().think(&player_state.clone(), None, Some(1_306_003))));
}
