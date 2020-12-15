use aoc::game::Game;
use aoc::util::load_input;
use std::str::FromStr;

fn main() {
    let lines = load_input("015").expect("could not load input");
    let mut game = Game::from_str(lines.first().expect("no lines in input"))
        .expect("could not initialize game");

    while game.get_turn() <= 2020 {
        game.take_turn()
            .expect(&format!("could not take turn {:#?}", game));
    }

    println!("part 1: {}", game.get_last_spoken());
    // while game.get_turn() <= 30_000_000 {
    //     game.take_turn().expect(&format!("could not take turn {:#?}", game));

    //     if game.get_turn() % 1_000 == 0 {
    //         println!("{}", game.get_last_spoken());
    //     }
    // }

    // println!("part 2: {}", game.get_last_spoken());
}
