use aoc::food::{Allergen, Ingredient, ShoppingList};
use aoc::util::load_input;
use criterion::{criterion_group, BenchmarkId, Criterion};

pub fn bench(c: &mut Criterion) {
    let lines = load_input("021").expect("could not load input");
    let list = ShoppingList::from_input(&lines).expect("could not parse shopping list");

    let mut group = c.benchmark_group("021 allergen assessment");
    group.bench_function(BenchmarkId::new("parts 1 and 2", "normal"), |b| {
        b.iter(|| {
            let map = list.match_allergens();
            let _: usize = map
                .iter()
                .filter(|(_, v)| v.is_none())
                .map(|(ingredient, _)| list.count_appearance(ingredient))
                .sum();
            let mut mapping = map
                .iter()
                .filter(|(_, v)| v.is_some())
                .map(|(ingredient, allergen)| (ingredient.clone(), allergen.clone().unwrap()))
                .collect::<Vec<(Ingredient, Allergen)>>();

            mapping.sort_by(|a, b| a.1 .0.cmp(&b.1 .0));

            mapping
                .iter()
                .map(|(i, _)| i.0.as_ref())
                .collect::<Vec<&str>>()
                .join(",")
        })
    });
    group.finish();
}

criterion_group!(benches, bench);
