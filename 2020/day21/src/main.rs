use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

#[derive(Debug, Default)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn main() -> Result<()> {
    let all_foods = read_input()?;
    let ingredients: HashSet<&str> = all_foods
        .iter()
        .flat_map(|f| f.ingredients.iter().map(|s| s.as_str()))
        .collect();
    let foods_by_ingredient = {
        let mut map = HashMap::<&str, Vec<&Food>>::new();
        for food in &all_foods {
            for ingredient in &food.ingredients {
                map.entry(ingredient).or_default().push(food);
            }
        }
        map
    };
    let foods_by_allergen = {
        let mut map = HashMap::<&str, Vec<&Food>>::new();
        for food in &all_foods {
            for allergen in &food.allergens {
                map.entry(allergen).or_default().push(food);
            }
        }
        map
    };

    let mut possible_ingredients_by_allergen = HashMap::<&str, HashSet<&str>>::new();
    for (allergen, foods) in &foods_by_allergen {
        let mut sets = foods
            .iter()
            .map(|f| &f.ingredients)
            .map(|i| i.iter().map(|s| s.as_str()).collect::<HashSet<&str>>());
        let first = sets.next().unwrap();

        possible_ingredients_by_allergen.insert(
            allergen,
            sets.fold(first, |a, b| a.intersection(&b).cloned().collect()),
        );
    }

    println!(
        "part 1: {}",
        ingredients
            .iter()
            .cloned()
            .filter(|i| !possible_ingredients_by_allergen
                .values()
                .any(|is| is.contains(i)))
            .map(|i| foods_by_ingredient[i].len())
            .sum::<usize>()
    );

    let mut remaining: HashSet<&str> = possible_ingredients_by_allergen.keys().cloned().collect();
    while !remaining.is_empty() {
        let (allergen, ingredient) = possible_ingredients_by_allergen
            .iter()
            .filter_map(|(allergen, ingredients)| {
                if remaining.contains(allergen) && ingredients.len() == 1 {
                    Some((*allergen, ingredients.iter().cloned().next().unwrap()))
                } else {
                    None
                }
            })
            .next()
            .unwrap();

        for (_, possible_ingredients) in possible_ingredients_by_allergen
            .iter_mut()
            .filter(|(&a, _)| a != allergen)
        {
            possible_ingredients.remove(ingredient);
        }

        remaining.remove(allergen);
    }

    let allergens_by_ingredient: HashMap<&str, &str> = possible_ingredients_by_allergen
        .iter()
        .map(|(allergen, ingredients)| {
            assert_eq!(ingredients.len(), 1);
            (ingredients.iter().next().cloned().unwrap(), *allergen)
        })
        .collect();

    let mut dangerous_ingredients: Vec<&str> = allergens_by_ingredient.keys().cloned().collect();

    dangerous_ingredients.sort_by_key(|ingredient| allergens_by_ingredient[ingredient]);

    println!("part 2: {}", dangerous_ingredients.join(","));

    Ok(())
}

fn read_input() -> anyhow::Result<Vec<Food>> {
    BufReader::new(File::open("input")?)
        .lines()
        .map(|r| {
            r.map_err(Into::into).map(|line| {
                let mut f = Food::default();
                let mut ingredient = true;

                for item in line.split(' ') {
                    if ingredient && item == "(contains" {
                        ingredient = false;
                    } else if ingredient {
                        f.ingredients.insert(item.into());
                    } else {
                        // Remove the trailing `,` or `)`.
                        f.allergens.insert(item[..item.len() - 1].into());
                    }
                }

                f
            })
        })
        .collect()
}
