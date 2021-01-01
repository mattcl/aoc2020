use aoc::lobby::{Address, Face, Lobby};
use aoc::util::load_input;
use criterion::{criterion_group, BenchmarkId, Criterion};

pub fn bench(c: &mut Criterion) {
    let lines = load_input("024").expect("could not load input");
    let mut lobby = Lobby::new();
    let addresses = Address::from_input(&lines).expect("could not load addresses");
    addresses.iter().for_each(|address| lobby.flip(address));

    let mut group = c.benchmark_group("024 lobby layout part 1");
    group.bench_function(BenchmarkId::new("creating lobby", "normal"), |b| {
        b.iter(|| {
            let mut lobby = Lobby::new();
            addresses.iter().for_each(|address| lobby.flip(address));
            lobby.count_tiles(&Face::Black);
        })
    });
    group.finish();

    let mut group = c.benchmark_group("024 lobby layout part 2");
    group.bench_function(BenchmarkId::new("simulating", "normal"), |b| {
        b.iter(|| {
            let mut lobby = lobby.clone();
            lobby.simulate(100);
            lobby.count_tiles(&Face::Black);
        })
    });
    group.finish();
}

criterion_group!(benches, bench);
