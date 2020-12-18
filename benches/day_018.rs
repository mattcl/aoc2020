use aoc::calculator::{StrParser, AdvancedParser, Parser};
use aoc::util::load_input;
use criterion::{criterion_group, BenchmarkId, Criterion};

pub fn bench(c: &mut Criterion) {
    let lines = load_input("018").expect("could not load input");

    let mut group = c.benchmark_group("018 operation order");
    group.bench_function(BenchmarkId::new("parser", "normal"), |b| {
        let parser = Parser {};
        b.iter(|| {
            lines.iter().map(|line| parser.eval(line)).sum::<i64>();
        })
    });

    group.bench_function(BenchmarkId::new("parser", "advanced"), |b| {
        let parser = AdvancedParser {};
        b.iter(|| {
            lines.iter().map(|line| parser.eval(line)).sum::<i64>();
        })
    });

    group.finish();
}

criterion_group!(benches, bench);
