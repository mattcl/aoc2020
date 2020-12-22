use aoc::combat::{Game, RecursiveGame};
use aoc::util::load_input;

fn main() {
    let lines = load_input("022").expect("could not load input");
    let mut game = Game::from_input(&lines).expect("could not create game");
    let winner = game.play().expect("could not play game");
    println!("part 1: {}", winner.0.score());

    let mut game = RecursiveGame::from_input(&lines).expect("could not create game");
    let winner = game.play().expect("could not play game");
    println!("part 2: {}", winner.0.score());

    let mut game = RecursiveGame::from_input(&lines).expect("could not create game");
    let winner = game.play_cached().expect("could not play game");
    println!("part 2: {}", winner.0.score());
}
