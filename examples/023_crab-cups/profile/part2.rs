use aoc::cups::Game;

fn main() {
    let mut game = Game::from_str_with_len("459672813", 1_000_000).expect("could not make game");
    game.simulate(10_000_000);
    println!("part 2: {}", game.crappy_checksum());
}
