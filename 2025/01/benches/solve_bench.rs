use advent_01::solve; // adapte au nom de ton crate
use criterion::{Criterion, criterion_group, criterion_main};
use once_cell::sync::Lazy;
use std::{hint::black_box, io::Cursor};

const EXAMPLE_INPUT: &str = include_str!("../src/input/example.txt");
const REAL_INPUT: &str = include_str!("../src/input/rotations.txt");
static BIG_INPUT: Lazy<String> = Lazy::new(|| "R1\n".repeat(1_000_000));

fn bench_solve_example_input(c: &mut Criterion) {
    c.bench_function("solve_on_example_input", |b| {
        b.iter(|| {
            let cursor = Cursor::new(EXAMPLE_INPUT);
            let res = solve(black_box(cursor)).unwrap();
            black_box(res);
        });
    });
}

fn bench_solve_real_input(c: &mut Criterion) {
    c.bench_function("solve_on_real_input", |b| {
        b.iter(|| {
            let cursor = Cursor::new(REAL_INPUT);
            let res = solve(black_box(cursor)).unwrap();
            black_box(res);
        });
    });
}

fn bench_solve_big_input(c: &mut Criterion) {
    c.bench_function("solve_on_big_synthetic_input", |b| {
        b.iter(|| {
            let cursor = Cursor::new(black_box(&**BIG_INPUT));
            let res = solve(cursor).unwrap();
            black_box(res);
        })
    });
}

criterion_group!(
    benches,
    bench_solve_example_input,
    bench_solve_real_input,
    bench_solve_big_input
);
criterion_main!(benches);
