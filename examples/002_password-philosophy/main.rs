use aoc::password::{count_valid_passwords, PolicyType};
use aoc::util::load_lines;


fn main() {
    let lines = load_lines("examples/002_password-philosophy/input").expect("could not load input");

    println!("part 1: {}", count_valid_passwords(&lines, &PolicyType::Count).expect("invalid input"));
    println!("part 2: {}", count_valid_passwords(&lines, &PolicyType::Position).expect("invalid input"));
}
