use aoc::message::{input_map, Ruleset};
use aoc::util::load_input;

fn main() {
    let lines = load_input("019").expect("could not load input");

    let mut parts = lines.split(|line| line.is_empty());

    let mut map = input_map(
        parts.next().expect("input lacking rules")
    ).expect("could not name input map");

    let ruleset = Ruleset::from_input_map(&map).expect("could not make ruleset");
    let messages = parts.next().expect("no remaining input");
    let res = messages.iter().filter(|line| ruleset.check(0, line)).count();

    println!("part 1: {}", res);

    // replace rules
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    map.entry((8 as usize)).and_modify(|e| *e = "42 | 42 8");
    map.entry((11 as usize)).and_modify(|e| *e = "42 31 | 42 11 31");

    let ruleset = Ruleset::from_input_map(&map).expect("could not make ruleset");
    let res = messages.iter().filter(|line| ruleset.check(0, line)).count();

    println!("part 2: {}", res);
}
