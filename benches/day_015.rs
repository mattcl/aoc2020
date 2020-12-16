use aoc::game::Game;
use aoc::util::load_input;
use criterion::{black_box, criterion_group, BenchmarkId, Criterion};
use std::str::FromStr;

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("015 rambunctious recitation part 1");
    let lines = load_input("015").expect("could not load input");

    group.bench_function(BenchmarkId::new("take turns", "normal"), |b| {
        let game = Game::from_str(lines.first().expect("no lines in input"))
            .expect("could not initialize game");

        b.iter(|| {
            let mut game = game.clone();
            while game.get_turn() <= 2020 {
                game.take_turn().expect("could not take turn");
            }
        })
    });

    group.finish();

    let mut group = c.benchmark_group("015 rambunctious recitation part 2");
    group.bench_function(BenchmarkId::new("take turns", "normal"), |b| {
        let game = Game::from_str(lines.first().expect("no lines in input"))
            .expect("could not initialize game");

        b.iter(|| {
            let mut game = game.clone();
            while game.get_turn() <= 30_000_000 {
                game.take_turn().expect("could not take turn");
            }
        })
    });

    group.finish();
}

criterion_group!(benches, bench);
