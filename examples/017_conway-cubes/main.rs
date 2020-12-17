use aoc::power::{Coordinate, FourDCoordinate, Grid};
use aoc::util::load_input;

fn main() {
    let lines = load_input("017").expect("could not load input");

    let mut grid: Grid<Coordinate> = Grid::from_input(&lines);

    grid.boot(6);

    println!("part 1: {}", grid.active());

    let mut grid: Grid<FourDCoordinate> = Grid::from_input(&lines);

    grid.boot(6);

    println!("part 2: {}", grid.active());
}
