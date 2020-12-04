use aoc::util::load_input;
use aoc::passport::Passport;
use criterion::{black_box, criterion_group, Criterion};

pub fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("004 passport processing");
    let example = vec![
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
        "".to_string(),
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929".to_string(),
        "".to_string(),
        "hcl:#ae17e1 iyr:2013".to_string(),
        "eyr:2024".to_string(),
        "ecl:brn pid:760753108 byr:1931".to_string(),
        "hgt:179cm".to_string(),
        "".to_string(),
        "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
        "iyr:2011 ecl:brn hgt:59in".to_string(),
    ];
    let actual = load_input("004").expect("Could not load input");

    group.bench_function("part 1 example", |b| {
        b.iter(|| Passport::from_input(black_box(&example))
                    .into_iter()
                    .filter(|passport| passport.is_ok())
                    .count())
    });
    group.bench_function("part 1 actual", |b| {
        b.iter(|| Passport::from_input(black_box(&actual))
                    .into_iter()
                    .filter(|passport| passport.is_ok())
                    .count())
    });
    group.bench_function("part 2 example", |b| {
        b.iter(|| Passport::from_input(black_box(&example))
                    .into_iter()
                    .filter(|passport| passport.is_ok())
                    .map(|passport| passport.unwrap())
                    .filter(|passport| passport.validate().is_ok())
                    .count())
    });
    group.bench_function("part 2 actual", |b| {
        b.iter(|| Passport::from_input(black_box(&actual))
                    .into_iter()
                    .filter(|passport| passport.is_ok())
                    .map(|passport| passport.unwrap())
                    .filter(|passport| passport.validate().is_ok())
                    .count())
    });
    group.finish();
}

criterion_group!(benches, bench);
