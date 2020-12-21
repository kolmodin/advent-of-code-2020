#![feature(iterator_fold_self)]

use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

struct Food<'a> {
    ingredients: Vec<&'a str>,
    allergens: Vec<&'a str>,
}

fn parse_line(inp: &str) -> Food {
    let parts: Vec<&str> = inp[0..inp.len() - 1].split(" (contains ").collect();
    Food {
        ingredients: parts[0].split(' ').collect::<Vec<_>>(),
        allergens: parts[1].split(", ").collect::<Vec<_>>(),
    }
}

fn main() {
    let contents = fs::read_to_string("day21.txt").expect("Something went wrong reading the file");

    let foods: Vec<Food> = contents.lines().map(parse_line).collect();

    let all_allergens: Vec<_> = foods
        .iter()
        .flat_map(|f| f.allergens.iter())
        .unique()
        .collect();

    println!("all allergens: {:?}", &all_allergens);

    let mut maybe_some_allergen: HashSet<&str> = HashSet::new();

    let mut allergen_to_candidate = HashMap::new();
    for allergen in &all_allergens {
        let food_with_allergen: Vec<HashSet<&str>> = foods
            .iter()
            .filter(|f| f.allergens.contains(allergen))
            .map(|f| f.ingredients.iter().cloned().collect::<HashSet<&str>>())
            .collect();
        let set = food_with_allergen
            .into_iter()
            .fold_first(|set, f| set.intersection(&f).cloned().collect())
            .unwrap();
        println!("Found {} {:?}", allergen, set);
        maybe_some_allergen.extend(set.clone());
        allergen_to_candidate.insert(allergen, set);
    }

    let count = foods
        .iter()
        .flat_map(|f| f.ingredients.iter())
        .filter(|i| !maybe_some_allergen.contains(*i))
        .count();

    println!("Part 1: {}", count);

    let mut known_allergens: Vec<(&str, &str)> = vec![];

    while !allergen_to_candidate.is_empty() {
        let read = allergen_to_candidate.clone();
        for (allergen, list) in read {
            if list.len() == 1 {
                let ingredient = list.iter().next().unwrap();
                known_allergens.push((allergen, ingredient));
                // Remove from all other candidate lists.
                for mut alg in allergen_to_candidate.values_mut() {
                    alg.remove(ingredient);
                }
                // This allergen is solved, remove it.
                allergen_to_candidate.remove(allergen);
            }
        }
    }

    known_allergens.sort();
    let part2_list: Vec<_> = known_allergens.into_iter().map(|(a, i)| i).collect();
    println!("Part 2: {:?}", part2_list.join(","));
}
