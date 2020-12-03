use aoc::util::load_input;
use aoc::toboggan::Forest;


fn main() {
    let lines = load_input("003").expect("could not load input");
    let forest = Forest::new(&lines).expect("Could not make Forest from input");
    println!("part 1: {}", forest.traverse(1, 3));


    let res = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
        .into_iter()
        .map(|(row, col)| forest.traverse(row, col))
        .fold(1, |acc, count| acc * count);

    println!("part 2: {}", res);
}
