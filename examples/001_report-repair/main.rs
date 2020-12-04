use aoc::expense::{expense_report, triple_expense};
use aoc::util::load_input;

fn main() {
    let vals: Vec<i32> = load_input("001")
        .expect("Could not load input")
        .into_iter()
        .map(|v| v.parse())
        .collect::<Result<Vec<i32>, _>>()
        .expect("Could not convert all input to i32");

    println!(
        "part 1: {}",
        expense_report(&vals, 2020).expect("Could not fix expense report")
    );
    println!(
        "part 2: {}",
        triple_expense(&vals, 2020).expect("Could not fix expense report")
    );
}
