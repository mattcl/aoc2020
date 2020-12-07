use aoc::luggage::Ruleset;
use aoc::util::load_input;

fn main() {
    let lines = load_input("007").expect("could not load input");
    let ruleset = Ruleset::from_input(&lines).expect("could not parse input");

    println!(
        "part 1: {}",
        ruleset
            .get_num_possible_bags("shiny gold")
            .expect("could not search for bag")
    );
    println!(
        "part 1: {}",
        ruleset
            .get_num_possible_bags_parallel("shiny gold")
            .expect("could not search for bag")
    );
    println!(
        "part 1: {}",
        ruleset
            .get_num_possible_bags_memoized("shiny gold")
            .expect("could not search for bag")
    );

    println!(
        "part 2: {}",
        ruleset
            .count_bags("shiny gold")
            .expect("could not count bags")
    );
    println!(
        "part 2: {}",
        ruleset
            .count_bags_memoized("shiny gold")
            .expect("could not count bags")
    );
}
