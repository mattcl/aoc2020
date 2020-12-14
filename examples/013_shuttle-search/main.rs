use aoc::shuttle::Schedule;
use aoc::util::load_input;
use std::str::FromStr;

fn main() {
    let lines = load_input("013").expect("could not load input");
    let mut lines = lines.iter();

    let start = lines.next().expect("no first line").parse::<usize>().expect("first line not a number");
    let schedule = Schedule::from_str(lines.next().expect("no second line")).expect("could not make schedule");

    let res = schedule.earliest_departure(start).expect("could not find bus");

    println!("part 1: {}", res.0 * res.1.id());

    println!("part 2: {}", schedule.sync_departures().expect("could not find sync departure"));
}
