#![feature(test)]
use p22::calc::{fibonacci_loop, fibonacci_rec, celsius2fahrenheit, fahrenheit2celsius};

extern crate test;
use test::{Bencher, black_box};

#[bench]
fn bench_fibonacci_loop(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..100 {
            black_box(fibonacci_loop(i));
        }
    });
}

#[bench]
fn bench_fibonacci_rec(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..10 {
            black_box(p22::calc::fibonacci_rec(i));
        }
    });
}

#[bench]
fn bench_celsius2fahrenheit(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..10 {
            black_box(p22::calc::celsius2fahrenheit(i));
        }
    });
}

#[bench]
fn bench_fahrenheit2celsius(b: &mut Bencher) {
    b.iter(|| {
        for i in 0..10 {
            black_box(p22::calc::fahrenheit2celsius(i));
        }
    });
}
