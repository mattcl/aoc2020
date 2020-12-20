use aoc::calculator::{AdvancedParser, Parser, StrParser};
use aoc::util::load_input;

fn main() {
    let lines = load_input("018").expect("could not load input");

    let parser = Parser {};
    let res: i64 = lines.iter().map(|line| parser.eval(line)).sum();
    println!("part 1: {}", res);

    let advanced_parser = AdvancedParser {};
    let res: i64 = lines.iter().map(|line| advanced_parser.eval(line)).sum();
    println!("part 2: {}", res);
}
