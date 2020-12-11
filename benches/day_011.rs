use aoc::seating::Area;
use aoc::util::load_input;
use criterion::{black_box, criterion_group, BenchmarkId, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("011 seating system");
    let lines = load_input("011").expect("could not load input");

    for dist in &[1, 10, 20, 30] {
        group.bench_with_input(
            BenchmarkId::new("step", format!("search distance {}", dist)),
            dist,
            |b, dist| {
                let area = Area::new(&lines, Some(*dist), 4).expect("could not load seating area");

                b.iter(|| {
                    let mut area = area.clone();
                    for _ in 0..20 {
                        area = area.step().unwrap();
                    }
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench);
