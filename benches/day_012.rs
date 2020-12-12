use aoc::navigation::{Plan, Ship, WaypointShip};
use aoc::util::load_input;
use criterion::{black_box, criterion_group, BenchmarkId, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("012 rain risk parts 1 & 2");
    let lines = load_input("012").expect("could not load input");
    let plan = Plan::from_input(&lines).expect("could not make plan");

    group.bench_function(BenchmarkId::new("execute", "ship"), |b| {
        b.iter(|| {
            let mut ship = Ship::new();
            plan.execute(&mut ship).expect("could not run plan")
        })
    });

    group.bench_function(BenchmarkId::new("execute", "waypoint ship"), |b| {
        b.iter(|| {
            let mut ship = WaypointShip::new();
            plan.execute(&mut ship).expect("could not run plan")
        })
    });

    group.finish();
}

criterion_group!(benches, bench);
