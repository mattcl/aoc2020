use aoc::console::Program;
use aoc::util::load_input;
use criterion::{black_box, criterion_group, BenchmarkId, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("008 handheld halting part 2");
    let lines = load_input("008").expect("could not load input");
    let mut program = Program::new(&lines).expect("could not load program");

    group.bench_function(BenchmarkId::new("correct", "iterative"), |b| {
        b.iter(|| program.correct())
    });

    group.bench_function(BenchmarkId::new("correct", "parallel"), |b| {
        b.iter(|| program.correct_parallel())
    });

    group.bench_function(BenchmarkId::new("correct", "recursive"), |b| {
        b.iter(|| program.correct_recursive())
    });

    group.finish();
}

criterion_group!(benches, bench);
