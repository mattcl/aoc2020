use aoc::error::Result;
use aoc::util::load_input;
use aoc::customs::Group;

fn main() {
    let lines = load_input("006").expect("could not load input");

    let res = Group::from_input(&lines)
        .into_iter()
        .collect::<Result<Vec<Group>>>()
        .expect("Could not load all groups");

    let unique_answers = res
        .iter()
        .map(|g| g.unique_answers())
        .sum::<usize>();

    println!("part 1: {}", unique_answers);

    let collective_answers = res
        .iter()
        .map(|g| g.collective_answers())
        .sum::<usize>();

    println!("part 2: {}", collective_answers);
}
