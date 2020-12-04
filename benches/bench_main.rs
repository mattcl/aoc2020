use criterion::criterion_main;

mod day_001;
mod day_002;
mod day_003;

criterion_main! {
    day_001::benches,
    day_002::benches,
    day_003::benches,
}
