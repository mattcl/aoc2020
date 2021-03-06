use criterion::criterion_main;

mod day_001;
mod day_002;
mod day_003;
mod day_004;
mod day_005;
mod day_007;
mod day_008;
mod day_009;
mod day_010;
mod day_011;
mod day_012;
mod day_014;
mod day_015;
mod day_016;
mod day_017;
mod day_018;
mod day_019;
mod day_020;
mod day_021;
mod day_022;
mod day_023;
mod day_024;
mod day_025;

criterion_main! {
    day_001::benches,
    day_002::benches,
    day_003::benches,
    day_004::benches,
    day_005::benches,
    day_007::benches,
    day_008::benches,
    day_009::benches,
    day_010::benches,
    day_011::benches,
    day_012::benches,
    day_014::benches,
    day_015::benches,
    day_016::benches,
    day_017::benches,
    day_018::benches,
    day_019::benches,
    day_020::benches,
    day_021::benches,
    day_022::benches,
    day_023::benches,
    day_024::benches,
    day_025::benches,
}
