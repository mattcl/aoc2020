use aoc::power::{Coordinate, FourDCoordinate, Grid};
use aoc::util::load_input;
use criterion::{criterion_group, BenchmarkId, Criterion};

pub fn bench(c: &mut Criterion) {
    let lines = load_input("017").expect("could not load input");

    let mut group = c.benchmark_group("017 conway cubes");
    group.bench_function(BenchmarkId::new("boot", "3d"), |b| {
        let grid: Grid<Coordinate> = Grid::from_input(&lines);

        b.iter(|| {
            let mut grid = grid.clone();
            grid.boot(6);
        })
    });

    group.bench_function(BenchmarkId::new("boot", "4d"), |b| {
        let grid: Grid<FourDCoordinate> = Grid::from_input(&lines);

        b.iter(|| {
            let mut grid = grid.clone();
            grid.boot(6);
        })
    });
    group.finish();

    let mut group = c.benchmark_group("017 conway cubes scaling");
    for i in (5..=15).step_by(5) {
        group.bench_with_input(BenchmarkId::new("boot 3d", i), &i, |b, i| {
            let grid: Grid<Coordinate> = Grid::from_input(&lines);

            b.iter(|| {
                let mut grid = grid.clone();
                grid.boot(*i);
            })
        });
        group.bench_with_input(BenchmarkId::new("boot 4d", i), &i, |b, i| {
            let grid: Grid<FourDCoordinate> = Grid::from_input(&lines);

            b.iter(|| {
                let mut grid = grid.clone();
                grid.boot(*i);
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench);
