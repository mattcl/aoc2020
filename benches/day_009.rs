use aoc::xmas::Document;
use aoc::util::load_input;
use criterion::{black_box, criterion_group, BenchmarkId, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("009 encoding error part 1");
    let lines = load_input("009").expect("could not load input");
    let document = Document::new(&lines).expect("could not load document");

    group.bench_function(BenchmarkId::new("find_outlier", "brute force"), |b| {
        b.iter(|| document.find_outlier(black_box(25)).unwrap())
    });

    group.finish();

    let outlier = document.find_outlier(25).unwrap();

    let mut group = c.benchmark_group("009 encoding error part 2");
    group.bench_function(BenchmarkId::new("find_weakness", "fast"), |b| {
        b.iter(|| document.find_weakness(black_box(outlier)).unwrap())
    });

    group.bench_function(BenchmarkId::new("find_weakness", "slow"), |b| {
        b.iter(|| document.find_weakness_slow(black_box(outlier)).unwrap())
    });

    group.finish();
}

criterion_group!(benches, bench);
