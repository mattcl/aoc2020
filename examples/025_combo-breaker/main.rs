use aoc::encryption::{Device, Key};
use aoc::util::load_input;

fn main() {
    let lines = load_input("025").expect("could not load input");
    let mut values = lines
        .iter()
        .map(|line| line.parse::<usize>())
        .collect::<Result<Vec<usize>, std::num::ParseIntError>>()
        .expect("could not parse input")
        .into_iter()
        .map(|v| Device::from_key(Key(v), 7));

    let device1 = values.next().expect("missing device 1");
    let device2 = values.next().expect("missing device 2");

    let encryption_key = device1.encryption_key(&device2.public_key);

    println!("part 1: {:?}", encryption_key);
}
