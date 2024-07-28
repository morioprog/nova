#![feature(test)]

use core::{
    board::Board,
    color::PuyoColor::*,
    player_state::PlayerState,
    tumo::{Tumo, Tumos},
};

use nova_newbot::{
    evaluator::BUILD,
    searcher::{BeamSearcher, Searcher},
};
use test::Bencher;

extern crate test;

// 31,104,760 ns/iter (+/- 2,006,063)
#[bench]
fn bench_beam_search_frame_2(b: &mut Bencher) {
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

    b.iter(|| {
        test::black_box(BeamSearcher::search(
            &player_state.clone(),
            &BUILD.clone(),
            Some(2),
        ))
    })
}

// 123,285,540 ns/iter (+/- 4,684,603)
#[bench]
fn bench_beam_search_frame_8(b: &mut Bencher) {
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

    b.iter(|| {
        test::black_box(BeamSearcher::search(
            &player_state.clone(),
            &BUILD.clone(),
            Some(8),
        ))
    })
}

// 362,722,770 ns/iter (+/- 11,512,221)
#[bench]
fn bench_beam_search_frame_24(b: &mut Bencher) {
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

    b.iter(|| {
        test::black_box(BeamSearcher::search(
            &player_state.clone(),
            &BUILD.clone(),
            Some(24),
        ))
    })
}
