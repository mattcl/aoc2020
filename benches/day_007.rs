use aoc::luggage::Ruleset;
use aoc::util::load_input;
use criterion::{black_box, criterion_group, BenchmarkId, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("007 handy haversacks part 1");
    let actual = load_input("007").expect("Could not load input");
    let ruleset = Ruleset::from_input(&actual).expect("Could not make ruleset");

    group.bench_function(BenchmarkId::new("get_num_possible_bags", "normal"), |b| {
        b.iter(|| ruleset.get_num_possible_bags(black_box("shiny gold")))
    });

    group.bench_function(BenchmarkId::new("get_num_possible_bags", "parallel"), |b| {
        b.iter(|| ruleset.get_num_possible_bags_parallel(black_box("shiny gold")))
    });

    group.bench_function(BenchmarkId::new("get_num_possible_bags", "memoized"), |b| {
        b.iter(|| ruleset.get_num_possible_bags_memoized(black_box("shiny gold")))
    });

    group.finish();

    let mut group = c.benchmark_group("007 handy haversacks part 2");
    group.bench_function(BenchmarkId::new("count_bags", "normal"), |b| {
        b.iter(|| ruleset.count_bags(black_box("shiny gold")))
    });

    group.bench_function(BenchmarkId::new("count_bags", "memoized"), |b| {
        b.iter(|| ruleset.count_bags_memoized(black_box("shiny gold")))
    });

    group.finish();
}

criterion_group!(benches, bench);
