use aoc::cups::Game;
use criterion::{criterion_group, BenchmarkId, Criterion};
use std::str::FromStr;

pub fn bench(c: &mut Criterion) {
    let game = Game::from_str("459672813").expect("could not make game");
    let long_game = Game::from_str_with_len("459672813", 1_000_000).expect("could not make game");

    let mut group = c.benchmark_group("023 crab cups part 1");
    group.bench_function(BenchmarkId::new("short game", "100"), |b| {
        b.iter(|| {
            let mut game = game.clone();
            game.simulate(100);
            let _ = game.order_string();
        })
    });
    group.finish();

    let mut group = c.benchmark_group("023 crab cups part 2");
    group.bench_function(BenchmarkId::new("long game", "10_000_000"), |b| {
        b.iter(|| {
            let mut game = long_game.clone();
            game.simulate(10_000_000);
            let _ = game.crappy_checksum();
        })
    });
    group.finish();
}

criterion_group!(benches, bench);
