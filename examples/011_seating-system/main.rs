use aoc::seating::Area;
use aoc::util::load_input;

fn main() {
    let lines = load_input("011").expect("could not load input");
    let mut area = Area::new(&lines, Some(1), 4).expect("Could not make seating area");

    loop {
        let new_area = area.step().expect("Could not simulate next step");

        if area == new_area {
            break;
        }

        area = new_area;
    }

    println!("part 1: {}", area.occupied_seats());

    let mut area = Area::new(&lines, None, 5).expect("Could not make seating area");
    loop {
        let new_area = area.step().expect("Could not simulate next step");

        if area == new_area {
            break;
        }

        area = new_area;
    }

    println!("part 2: {}", area.occupied_seats());
}
