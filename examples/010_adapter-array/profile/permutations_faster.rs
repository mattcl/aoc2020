use aoc::adapter::{permutations_faster, Adapter};
use aoc::util::load_input;

fn main() {
    let lines = load_input("010").expect("could not load input");
    let adapters = Adapter::from_input(&lines).expect("could not load adapters");
    println!(
        "part 2: {}",
        permutations_faster(&adapters).expect("could not count permutation")
    );
}
