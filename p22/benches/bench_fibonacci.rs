#![feature(test)]

extern crate test;
use test::Bencher;

use p22::calc::{fibonacci_loop, fibonacci_rec};

#[bench]
fn bench_fibonacci_loop(benches: &mut Bencher) {
    benches.iter(|| fibonacci_loop(100));
}

#[bench]
fn bench_fibonacci_rec(benches: &mut Bencher) {
    benches.iter(|| fibonacci_rec(100));
}
