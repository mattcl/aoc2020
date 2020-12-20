use aoc::jigsaw::{Grid, Tile};
use aoc::util::{load_input, test_input};
use criterion::{criterion_group, BenchmarkId, Criterion};
use std::collections::HashMap;

pub fn bench(c: &mut Criterion) {
    let lines = load_input("020").expect("could not load input");
    let grid = Grid::from_input(&lines).expect("could not construct grid");

    let mut group = c.benchmark_group("020 jurassic jigsaw part 1");
    group.bench_function(
        BenchmarkId::new("find correct arrangement", "normal"),
        |b| {
            b.iter(|| {
                let mut grid = grid.clone();
                grid.arrange();
                grid.get_corner_product()
                    .expect("could not calculate corner product");
            })
        },
    );

    group.finish();

    let shape = test_input(
        "
        ..................#.
        #....##....##....###
        .#..#..#..#..#..#...
    ",
    )
    .iter()
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

    let mut grid = grid.clone();
    grid.arrange();
    grid.get_corner_product()
        .expect("could not calculate corner product");

    let mut group = c.benchmark_group("020 jurassic jigsaw part 2");
    group.bench_function(BenchmarkId::new("find sea monsters", "normal"), |b| {
        b.iter(|| {
            let combined = grid
                .make_complete_tile()
                .expect("could not form complete tile");

            let mut tiles: HashMap<usize, Tile> = HashMap::new();
            tiles.insert(combined.id, combined);
            let mut combined_grid = Grid::new(&tiles);

            for (_, tile) in combined_grid.variant_map.iter_mut() {
                if tile.find_shape(&shape) > 0 {
                    break;
                }
            }
        })
    });
    group.finish();
}

criterion_group!(benches, bench);
