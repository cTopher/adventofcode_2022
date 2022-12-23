use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day16::{part_1, part_2};

const INPUT: &str = include_str!("../input.txt");

// time:   [10.567 ms 10.595 ms 10.629 ms]
pub fn part_1_benchmark(c: &mut Criterion) {
    c.bench_function("part 1", |b| b.iter(|| part_1(black_box(INPUT))));
}

// time:   [9.9215 ms 10.187 ms 10.507 ms]
pub fn part_2_benchmark(c: &mut Criterion) {
    c.bench_function("part 2", |b| b.iter(|| part_2(black_box(INPUT))));
}

criterion_group!(benches, part_1_benchmark, part_2_benchmark);
criterion_main!(benches);
