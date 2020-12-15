use aoc::docking::Initializer;
use aoc::util::load_input;
use criterion::{black_box, criterion_group, BenchmarkId, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("014 docking data part 1");
    let lines = load_input("014").expect("could not load input");

    group.bench_function(BenchmarkId::new("initialize version 1", "normal"), |b| {
        b.iter(|| {
            Initializer::initialize(black_box(&lines))
                .unwrap()
                .memory_sum()
        })
    });

    group.finish();

    let mut group = c.benchmark_group("014 docking data part 2");
    group.bench_function(BenchmarkId::new("initialize version 2", "normal"), |b| {
        b.iter(|| {
            Initializer::initialize_v2(black_box(&lines))
                .unwrap()
                .memory_sum()
        })
    });

    group.bench_function(BenchmarkId::new("initialize version 2", "memoized"), |b| {
        b.iter(|| {
            Initializer::initialize_v2_memoized(black_box(&lines))
                .unwrap()
                .memory_sum()
        })
    });

    group.finish();
}

criterion_group!(benches, bench);
