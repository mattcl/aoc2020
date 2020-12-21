use aoc::food::{Allergen, Ingredient, ShoppingList};
use aoc::util::load_input;

fn main() {
    let lines = load_input("021").expect("could not load input");
    let list = ShoppingList::from_input(&lines).expect("could not parse shopping list");
    let map = list.match_allergens();

    let res: usize = map
        .iter()
        .filter(|(_, v)| v.is_none())
        .map(|(ingredient, _)| list.count_appearance(ingredient))
        .sum();

    println!("part 1: {}", res);

    let mut mapping = map
        .iter()
        .filter(|(_, v)| v.is_some())
        .map(|(ingredient, allergen)| (ingredient.clone(), allergen.clone().unwrap()))
        .collect::<Vec<(Ingredient, Allergen)>>();

    mapping.sort_by(|a, b| a.1.0.cmp(&b.1.0));

    let res = mapping.iter().map(|(i, _)| i.0.as_ref()).collect::<Vec<&str>>().join(",");

    println!("part 2: {}", res);
}
