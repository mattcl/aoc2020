use aoc::xmas::Document;
use aoc::util::load_input;

fn main() {
    let lines = load_input("009").expect("could not load input");
    let document = Document::new(&lines).expect("could not parse input");

    let outlier = document.find_outlier(25).expect("Could not find outlier");

    println!("part 1: {}", outlier);
    println!("part 2: {}", document.find_weakness(outlier).expect("Could not find weakness"));
    println!("part 2: {}", document.find_weakness_slow(outlier).expect("Could not find weakness"));
}
