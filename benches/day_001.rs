use aoc::expense;
use aoc::util::load_input;
use criterion::{black_box, criterion_group, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("001 expense report");
    let example = vec![1721, 979, 366, 299, 675, 1456];
    let actual: Vec<i64> = load_input("001")
        .expect("Could not load input")
        .into_iter()
        .map(|v| v.parse())
        .collect::<Result<Vec<i64>, _>>()
        .expect("Could not convert all input to i64");

    group.bench_function("part 1 example", |b| {
        b.iter(|| expense::expense_report(black_box(&example), 2020))
    });
    group.bench_function("part 1 actual", |b| {
        b.iter(|| expense::expense_report(black_box(&actual), 2020))
    });
    group.bench_function("part 2 example", |b| {
        b.iter(|| expense::triple_expense(black_box(&example), 2020))
    });
    group.bench_function("part 2 actual", |b| {
        b.iter(|| expense::triple_expense(black_box(&actual), 2020))
    });
    group.finish();
}

criterion_group!(benches, bench);
