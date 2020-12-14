use aoc::docking::Initializer;
use aoc::util::load_input;

fn main() {
    let lines = load_input("014").expect("could not load input");
    let program = Initializer::initialize(&lines).expect("could not initialize");

    println!("part 1: {}", program.memory_sum());

    let program = Initializer::initialize_v2(&lines).expect("could not initialize");
    println!("part 2: {}", program.memory_sum());
}
