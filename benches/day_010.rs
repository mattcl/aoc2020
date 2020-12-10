use aoc::adapter::{compute_diffs_in_chain, permutations, permutations_faster, Adapter};
use aoc::util::load_input;
use criterion::{black_box, criterion_group, BenchmarkId, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("010 adapter array part 1");
    let lines = load_input("010").expect("could not load input");
    let adapters = Adapter::from_input(&lines).expect("could not load adapters");

    group.bench_function(
        BenchmarkId::new("compute_diffs_in_chain", "brute force"),
        |b| b.iter(|| compute_diffs_in_chain(black_box(&adapters)).unwrap()),
    );

    group.finish();

    let mut group = c.benchmark_group("010 adapter array part 2");
    group.bench_function(BenchmarkId::new("permutations", "fast"), |b| {
        b.iter(|| permutations(black_box(&adapters)).unwrap())
    });

    group.bench_function(BenchmarkId::new("permutations", "faster"), |b| {
        b.iter(|| permutations_faster(black_box(&adapters)).unwrap())
    });

    group.finish();
}

criterion_group!(benches, bench);
