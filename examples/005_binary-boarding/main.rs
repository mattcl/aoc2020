use aoc::boarding::{find_highest_id, Pass, Seat};
use aoc::error::Result;
use aoc::util::load_input;

fn main() {
    let lines = load_input("005").expect("could not load input");
    let mut res = lines
        .into_iter()
        .map(|line| Pass::new(&line))
        .collect::<Result<Vec<Pass>>>()
        .expect("Failed to load all passes")
        .into_iter()
        .map(|pass| pass.seat())
        .collect::<Result<Vec<Seat>>>()
        .expect("Failed to find all seats")
        .into_iter()
        .map(|seat| seat.id())
        .collect::<Vec<usize>>();

    println!("part 1: {}", res.iter().max().expect("Could not find max"));

    let lines = load_input("005").expect("could not load input");
    println!(
        "part 1: {}",
        find_highest_id(&lines).expect("Could not find max")
    );

    res.sort();
    let seats = res.split_off(1);
    let mut prev = res.pop().expect("First element should have existed");

    for id in seats {
        if id - prev == 2 {
            let s = Seat::from_id(id - 1);
            println!("part 2: {:?}, {}", s, s.id());
        }
        prev = id;
    }
}
