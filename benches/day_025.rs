use aoc::encryption::{Device, Key};
use aoc::util::load_input;
use criterion::{criterion_group, BenchmarkId, Criterion};

pub fn bench(c: &mut Criterion) {
    let lines = load_input("025").expect("could not load input");
    let values = lines
        .iter()
        .map(|line| line.parse::<usize>())
        .collect::<Result<Vec<usize>, std::num::ParseIntError>>()
        .expect("could not parse input");

    let mut group = c.benchmark_group("025 combo breaker");
    group.bench_function(BenchmarkId::new("finding encryption key", "normal"), |b| {
        b.iter(|| {
            let mut values = values.iter().map(|v| Device::from_key(Key(*v), 7));

            let device1 = values.next().expect("missing device 1");
            let device2 = values.next().expect("missing device 2");

            let _ = device1.encryption_key(&device2.public_key);
        })
    });
    group.finish();
}

criterion_group!(benches, bench);
