use aoc::console::Program;
use aoc::util::load_input;

fn main() {
    let lines = load_input("008").expect("could not load input");
    let mut program = Program::new(&lines).expect("could not load program");
    println!("{}", program.correct_recursive().expect("could not correct program"));
}
