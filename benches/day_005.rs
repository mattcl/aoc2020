use aoc::boarding::{
    find_highest_id,
    find_highest_id_bad_errors,
    find_highest_id_par_bad_errors,
    find_highest_id_par
};
use aoc::util::load_input;
use criterion::{black_box, criterion_group, BenchmarkId, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("005 binary boarding");
    let actual = load_input("005").expect("Could not load input");

    group.bench_function(BenchmarkId::new("find_highest", "normal"), |b| {
        b.iter(|| find_highest_id(black_box(&actual)))
    });
    group.bench_function(BenchmarkId::new("find_highest/bad handling", "normal"), |b| {
        b.iter(|| find_highest_id_bad_errors(black_box(&actual)))
    });
    group.bench_function(BenchmarkId::new("find_highest", "parallel"), |b| {
        b.iter(|| find_highest_id_par(black_box(&actual)))
    });
    group.bench_function(BenchmarkId::new("find_highest/bad handling", "parallel"), |b| {
        b.iter(|| find_highest_id_par_bad_errors(black_box(&actual)))
    });
    group.finish();
}

criterion_group!(benches, bench);
