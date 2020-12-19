use aoc::message::get_matching_messages_b;
use aoc::util::{load_input, load_named_input};

fn main() {
    let lines = load_input("019").expect("could not load input");
    let res = get_matching_messages_b(&lines).expect("could operate on input");
    println!("part 1: {}", res.len());

    let lines = load_named_input("019", "input_2").expect("could not load input");
    let res = get_matching_messages_b(&lines).expect("could operate on input");
    println!("part 2: {}", res.len());
}
