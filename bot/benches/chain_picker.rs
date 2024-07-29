#![feature(test)]

extern crate test;

use core::{
    board::Board,
    color::PuyoColor::*,
    player_state::PlayerState,
    tumo::{Tumo, Tumos},
};

use nova_bot::chain_picker::{enumerate_fireable_chains, strategies::*, ChainPicker};
use test::Bencher;

// 5,842,780 ns/iter (+/- 725,681)
// ~= 5.8 ms/iter (+/- 0.7)
#[bench]
fn bench_enumerate_fireable_chains(b: &mut Bencher) {
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
    let player_state = PlayerState::new(board, tumos, 0, 0, 0, 0, 0, 0);

    b.iter(|| test::black_box(enumerate_fireable_chains(&player_state.clone())));
}

// 4,972,535 ns/iter (+/- 491,759)
// ~= 5.0 ms/iter (+/- 0.5)
#[bench]
fn bench_enumerate_fireable_chains_empty(b: &mut Bencher) {
    let board = Board::from(concat!(
        "RGRGRG", // 2
        "GRGRGR", // 1
    ));
    let tumos = Tumos::new(&[
        Tumo::new(BLUE, YELLOW),
        Tumo::new(BLUE, YELLOW),
        Tumo::new(BLUE, YELLOW),
    ]);
    let player_state = PlayerState::new(board, tumos, 0, 0, 0, 0, 0, 0);

    b.iter(|| test::black_box(enumerate_fireable_chains(&player_state.clone())));
}

// 690 ns/iter (+/- 18)
#[bench]
fn bench_strategies_houwa(b: &mut Bencher) {
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
    let player_state_1p = PlayerState::new(board.clone(), tumos.clone(), 0, 0, 0, 0, 0, 0);
    let player_state_2p = PlayerState::new(board.clone(), tumos.clone(), 0, 0, 0, 0, 0, 0);
    let decisions = enumerate_fireable_chains(&player_state_1p.clone());

    b.iter(|| {
        test::black_box(Houwa::pick_chain(
            &player_state_1p.clone(),
            Some(&player_state_2p.clone()),
            &decisions.clone(),
        ))
    });
}
