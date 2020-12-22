use aoc::combat::RecursiveGame;
use aoc::util::load_input;

fn main() {
    let lines = load_input("022").expect("could not load input");
    let mut game = RecursiveGame::from_input(&lines).expect("could not create game");
    let winner = game.play().expect("could not play game");
    println!("part 2: {}", winner.0.score());
}
