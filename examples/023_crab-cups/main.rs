use aoc::cups::Game;
use std::str::FromStr;


fn main() {
    let mut game = Game::from_str("459672813").expect("could not make game");

    game.simulate(100);

    println!("part 1: {}", game.order_string());

    let mut game = Game::from_str_with_len("459672813", 1_000_000).expect("could not make game");
    game.simulate(10_000_000);
    println!("part 2: {}", game.crappy_checksum());
}
