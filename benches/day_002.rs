use aoc::password::{count_valid_passwords, PolicyType};
use aoc::util::load_input;
use criterion::{black_box, criterion_group, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("002 password philosophy");
    let example = vec![
        "1-3 a: abcde".to_string(),
        "1-3 b: cdefg".to_string(),
        "2-9 c: ccccccccc".to_string(),
    ];
    let actual = load_input("002").expect("Could not load input");

    group.bench_function("part 1 example", |b| {
        b.iter(|| count_valid_passwords(black_box(&example), &PolicyType::Count))
    });
    group.bench_function("part 1 actual", |b| {
        b.iter(|| count_valid_passwords(black_box(&actual), &PolicyType::Count))
    });
    group.bench_function("part 2 example", |b| {
        b.iter(|| count_valid_passwords(black_box(&example), &PolicyType::Position))
    });
    group.bench_function("part 2 actual", |b| {
        b.iter(|| count_valid_passwords(black_box(&actual), &PolicyType::Position))
    });
    group.finish();
}

criterion_group!(benches, bench);
