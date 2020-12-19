use aoc::message::{get_matching_messages, get_matching_messages_b};
use aoc::util::{load_input, load_named_input};
use criterion::{criterion_group, BenchmarkId, Criterion};

pub fn bench(c: &mut Criterion) {
    let lines = load_input("019").expect("could not load input");

    let mut group = c.benchmark_group("019 monster message part 1");

    group.bench_function(BenchmarkId::new("find matching messages", "dfs"), |b| {
        b.iter(|| {
            get_matching_messages(&lines).unwrap();
        })
    });

    group.bench_function(BenchmarkId::new("find matching messages", "bfs"), |b| {
        b.iter(|| {
            get_matching_messages_b(&lines).unwrap();
        })
    });

    group.finish();

    let lines = load_named_input("019", "input_2").expect("could not load input");
    let mut group = c.benchmark_group("019 monster message part 2");
    group.bench_function(BenchmarkId::new("find matching messages", "bfs"), |b| {
        b.iter(|| {
            get_matching_messages_b(&lines).unwrap();
        })
    });
    group.finish();
}

criterion_group!(benches, bench);
