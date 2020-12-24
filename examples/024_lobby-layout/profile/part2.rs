use aoc::lobby::{Address, Face, Lobby};
use aoc::util::load_input;

fn main() {
    let lines = load_input("024").expect("could not load input");
    let mut lobby = Lobby::new();
    let addresses = Address::from_input(&lines).expect("could not load addresses");
    addresses.iter().for_each(|address| lobby.flip(address));
    lobby.simulate(100);
    println!("part 2: {}", lobby.count_tiles(&Face::Black));
}
