use aoc::jigsaw::{Grid, Tile};
use aoc::util::{load_input, test_input};
use std::collections::HashMap;

fn main() {
    let lines = load_input("020").expect("could not load input");
    let mut grid = Grid::from_input(&lines).expect("could not construct grid");

    grid.arrange();

    println!(
        "part 1: {}",
        grid.get_corner_product()
            .expect("could not calculate corner product")
    );

    let shape = test_input(
        "
        ..................#.
        #....##....##....###
        .#..#..#..#..#..#...
    ",
    )
    .iter()
    .map(|line| line.chars().collect::<Vec<char>>())
    .collect::<Vec<Vec<char>>>();

    let combined = grid
        .make_complete_tile()
        .expect("could not form complete tile");

    // so we can leverage the fact that we already calculate the variants
    // when constructing a new Grid, so just make a new grid of just one tile
    let mut tiles: HashMap<usize, Tile> = HashMap::new();
    tiles.insert(combined.id, combined);
    let mut combined_grid = Grid::new(&tiles);

    for (_, tile) in combined_grid.variant_map.iter_mut() {
        if tile.find_shape(&shape) > 0 {
            println!("part 2: {}", tile.count_char('#'));
            break;
        }
    }
}
