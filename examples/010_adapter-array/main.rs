use aoc::adapter::{compute_diffs_in_chain, permutations, permutations_faster, Adapter};
use aoc::util::load_input;

fn main() {
    let lines = load_input("010").expect("could not load input");
    let adapters = Adapter::from_input(&lines).expect("could not load adapters");

    println!(
        "part 1: {}",
        compute_diffs_in_chain(&adapters).expect("could not make chain")
    );
    println!(
        "part 2: {}",
        permutations(&adapters).expect("could not count permutation")
    );
    println!(
        "part 2: {}",
        permutations_faster(&adapters).expect("could not count permutation")
    );
}
