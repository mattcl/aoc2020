use crate::error::{AocError, Result};
use std::collections::{HashMap, HashSet};


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Allergen(pub String);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Ingredient(pub String);

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Food {
    id: usize,
    ingredients: HashSet<Ingredient>,
    allergens: HashSet<Allergen>,
}

impl Food {
    pub fn new(id: usize) -> Self {
        Food {
            id: id,
            ingredients: HashSet::new(),
            allergens: HashSet::new(),
        }
    }

    pub fn contains_ingredient(&self, ingredient: &Ingredient) -> bool {
        self.ingredients.contains(ingredient)
    }

    pub fn contains_allergen(&self, allergen: &Allergen) -> bool {
        self.allergens.contains(allergen)
    }

    pub fn allergens(&self) -> &HashSet<Allergen> {
        &self.allergens
    }

    pub fn ingredients(&self) -> &HashSet<Ingredient> {
        &self.ingredients
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ShoppingList {
    food: Vec<Food>,
    allergens: HashSet<Allergen>,
    ingredients: HashSet<Ingredient>,
}

impl ShoppingList {
    pub fn new() -> Self {
        ShoppingList {
            food: Vec::new(),
            allergens: HashSet::new(),
            ingredients: HashSet::new(),
        }
    }

    pub fn from_input(input: &[String]) -> Result<Self> {
        let mut list = Self::new();

        for (index, line) in input.iter().enumerate() {
            let mut parts = line.split(" (contains ");

            // ingredients
            if let Some(ingredients_str) = parts.next() {
                let mut food = Food::new(index);
                for i in ingredients_str.split(' ') {
                    let ingredient = Ingredient(i.to_string());

                    list.ingredients.insert(ingredient.clone());
                    food.ingredients.insert(ingredient);
                }

                // alergens
                if let Some(allergens_str) = parts.next() {
                    for a in allergens_str.replace(")", "").split(", ") {
                        let allergen = Allergen(a.to_string());

                        list.allergens.insert(allergen.clone());
                        food.allergens.insert(allergen);
                    }
                }

                if food.ingredients.is_empty() {
                    return Err(AocError::InvalidInput("Food had no ingredients".to_string()));
                }

                list.food.push(food);
            }
        }

        Ok(list)
    }

    pub fn match_allergens(&self) -> HashMap<Ingredient, Option<Allergen>> {
        let mut candidate_map = self.make_candidate_map();
        let mut ingredient_map: HashMap<Ingredient, Option<Allergen>> = HashMap::new();
        self.ingredients.iter().for_each(|i| {
            match ingredient_map.insert(i.clone(), None) {
                _ => {}
            }
        });

        loop {
            if self.reduce(&mut candidate_map, &mut ingredient_map) {
                break;
            }
        }

        ingredient_map
    }

    pub fn reduce(
        &self,
        candidate_map: &mut HashMap<Allergen, HashSet<Ingredient>>,
        ingredient_map: &mut HashMap<Ingredient, Option<Allergen>>
    ) -> bool {
        let found = candidate_map
            .iter()
            .filter(|(_, ingredients)| ingredients.len() == 1)
            .map(|(allergen, ingredients)| {
                (allergen.clone(), ingredients.iter().cloned().next().unwrap())
            }).collect::<Vec<(Allergen, Ingredient)>>();

        if found.is_empty() {
            return true;
        }

        for (allergen, ingredient) in found.iter() {
            for (_, ingredients) in candidate_map.iter_mut().filter(|(_, ingredients)| ingredients.len() > 1) {
                ingredients.remove(&ingredient);
            }

            candidate_map.remove(&allergen);
            *ingredient_map.entry(ingredient.clone()).or_insert(None) = Some(allergen.clone());
        }

        false
    }

    fn make_candidate_map(&self) -> HashMap<Allergen, HashSet<Ingredient>> {
        let mut candidate_map: HashMap<Allergen, HashSet<Ingredient>> = HashMap::new();

        for allergen in self.allergens.iter() {
            let ingredients = self.food
                .iter()
                .filter(|f| f.contains_allergen(allergen))
                .fold(self.ingredients.clone(), |acc, f| {
                    acc.intersection(f.ingredients()).cloned().collect::<HashSet<Ingredient>>()
                });

            if !ingredients.is_empty() {
                candidate_map.insert(allergen.clone(), ingredients);
            }
        }

        candidate_map
    }

    pub fn count_appearance(&self, ingredient: &Ingredient) -> usize {
        self.food.iter().filter(|f| f.contains_ingredient(ingredient)).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_input;

    mod shopping_list {
        use super::*;

        fn example_list() -> ShoppingList {
            let input = test_input("
                mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
                trh fvjkl sbzzf mxmxvkd (contains dairy)
                sqjhc fvjkl (contains soy)
                sqjhc mxmxvkd sbzzf (contains fish)
            ");

            ShoppingList::from_input(&input).unwrap()
        }

        #[test]
        fn from_input() {
            let list = example_list();
            assert_eq!(list.food.len(), 4);
            assert_eq!(list.ingredients.len(), 7);
            assert_eq!(list.allergens.len(), 3);
        }

        #[test]
        fn make_candidate_map() {
            let list = example_list();
            let map = list.make_candidate_map();
            assert_eq!(map.len(), 3);
        }

        #[test]
        fn match_allergens() {
            let list = example_list();
            let map = list.match_allergens();

            let safe = vec!["kfcds", "nhms", "sbzzf", "trh"]
                .iter()
                .map(|s| Ingredient(s.to_string()))
                .collect::<Vec<Ingredient>>();

            let dangerous = vec!["mxmxvkd", "sqjhc", "fvjkl"]
                .iter()
                .map(|s| Ingredient(s.to_string()))
                .collect::<Vec<Ingredient>>();
            for i in safe.iter() {
                assert!(map.get(&i).unwrap().is_none());
            }

            for i in dangerous.iter() {
                assert!(map.get(&i).unwrap().is_some());
            }
        }

        #[test]
        fn count_appearance() {
            let list = example_list();
            assert_eq!(list.count_appearance(&Ingredient("mxmxvkd".to_string())), 3);
        }
    }
}
