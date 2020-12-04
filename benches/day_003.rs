use aoc::toboggan::Forest;
use aoc::util::load_input;
use criterion::{black_box, criterion_group, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("003 toboggan trajectory");
    let example = vec![
        "..##.......".to_string(),
        "#...#...#..".to_string(),
        ".#....#..#.".to_string(),
        "..#.#...#.#".to_string(),
        ".#...##..#.".to_string(),
        "..#.##.....".to_string(),
        ".#.#.#....#".to_string(),
        ".#........#".to_string(),
        "#.##...#...".to_string(),
        "#...##....#".to_string(),
        ".#..#...#.#".to_string(),
    ];

    let actual = load_input("003").expect("Could not load input");

    let forest_example = Forest::new(&example).expect("Could not create Forest");
    let forest_actual = Forest::new(&actual).expect("Could not create Forest");

    group.bench_function("part 1 example", |b| {
        b.iter(|| forest_example.traverse(black_box(1), black_box(3)))
    });
    group.bench_function("part 1 actual", |b| {
        b.iter(|| forest_actual.traverse(black_box(1), black_box(3)))
    });
    group.bench_function("part 2 example", |b| {
        b.iter(|| {
            vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
                .into_iter()
                .map(|(row, col)| forest_example.traverse(row, col))
                .fold(1, |acc, count| acc * count);
        })
    });
    group.bench_function("part 2 actual", |b| {
        b.iter(|| {
            vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
                .into_iter()
                .map(|(row, col)| forest_actual.traverse(row, col))
                .fold(1, |acc, count| acc * count);
        })
    });
    group.finish();
}

criterion_group!(benches, bench);
