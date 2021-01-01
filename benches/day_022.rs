use aoc::combat::{Game, RecursiveGame};
use aoc::util::load_input;
use criterion::{criterion_group, BenchmarkId, Criterion};

pub fn bench(c: &mut Criterion) {
    let lines = load_input("022").expect("could not load input");
    let game = Game::from_input(&lines).expect("could not create game");
    let recursive_game = RecursiveGame::from_input(&lines).expect("could not create game");

    let mut group = c.benchmark_group("022 crab combat");
    group.bench_function(BenchmarkId::new("combat", "normal"), |b| {
        b.iter(|| {
            let mut game = game.clone();
            game.play().unwrap();
        })
    });
    group.bench_function(BenchmarkId::new("recursive combat", "normal"), |b| {
        b.iter(|| {
            let mut game = recursive_game.clone();
            game.play().unwrap();
        })
    });
    group.bench_function(BenchmarkId::new("recursive combat", "cached"), |b| {
        b.iter(|| {
            let mut game = recursive_game.clone();
            game.play_cached().unwrap();
        })
    });
    group.finish();
}

criterion_group!(benches, bench);
