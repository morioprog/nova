#![feature(test)]

extern crate nova_core;
extern crate test;

use nova_core::board::Board;
use test::Bencher;

// AVX2   : 357 ns/iter (+/- 14)
// AVX512 : 316 ns/iter (+/- 8)
#[bench]
fn simulate_19chain(b: &mut Bencher) {
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

    b.iter(|| test::black_box(board.clone().simulate()))
}
