use aoc::game::Game;
use aoc::util::load_input;
use std::str::FromStr;

fn main() {
    let lines = load_input("015").expect("could not load input");
    let mut game = Game::from_str(lines.first().expect("no lines in input"))
        .expect("could not initialize game");

    while game.get_turn() <= 30_000_000 {
        game.take_turn().expect("could not take turn");
    }
}
