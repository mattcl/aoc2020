use aoc::console::Program;
use aoc::util::load_input;

fn main() {
    let lines = load_input("008").expect("could not load input");
    let mut program = Program::new(&lines).expect("could not load program");

    println!(
        "part 1: {}",
        program.execute().expect("could not run program").0
    );
    println!(
        "part 2: {}",
        program.correct().expect("could not correct program")
    );
    println!(
        "part 2: {}",
        program
            .correct_recursive()
            .expect("could not correct program")
    );
}
