use aoc::navigation::{Plan, Ship, WaypointShip};
use aoc::util::load_input;

fn main() {
    let lines = load_input("012").expect("could not load input");
    let plan = Plan::from_input(&lines).expect("could not make plan");

    let mut ship = Ship::new();
    plan.execute(&mut ship).expect("could not execute plan");
    println!("part 1: {}", ship.manhattan_distance());

    let mut ship = WaypointShip::new();
    plan.execute(&mut ship).expect("could not execute plan");
    println!("part 2: {}", ship.manhattan_distance());
}
