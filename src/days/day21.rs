use crate::prelude::*;

use std::collections::{HashMap, HashSet};

type Row = (HashSet<String>, HashSet<String>);

pub enum Day21 {}

fn parse_line(s: &str) -> Row {
    let mut words = s.split(' ');
    let mut ingredients = HashSet::new();
    for ingredient in &mut words {
        if ingredient == "(contains" {
            break;
        }
        ingredients.insert(ingredient.into());
    }
    let allergens = words
        .map(|x| x.strip_suffix(|c| ",)".contains(c)).unwrap_or(x).into())
        .collect();
    (ingredients, allergens)
}

fn shared(
    input: &<Day21 as Challenge>::Input,
) -> (HashSet<String>, HashMap<String, HashSet<String>>) {
    let (all_ingredients, all_allergens) = {
        let (mut ingredients, mut allergens) = (HashSet::new(), HashSet::new());
        for (i, a) in input {
            ingredients.extend(i.iter().cloned());
            allergens.extend(a.iter().cloned());
        }
        (ingredients, allergens)
    };

    let mut mappings = all_allergens
        .iter()
        .map(|k| (k.clone(), all_ingredients.clone()))
        .collect::<HashMap<String, HashSet<String>>>();

    for (ingr, alle) in input {
        for a in alle {
            let m = mappings.get_mut(a).unwrap();
            *m = &*m & ingr;
        }
    }

    let mut safe_ingredients = all_ingredients.clone();
    for ingr in mappings.values() {
        safe_ingredients = &safe_ingredients - ingr;
    }

    (safe_ingredients, mappings)
}

impl Challenge for Day21 {
    type Input = Vec<Row>;
    type Output1 = usize;
    type Output2 = String;

    fn read(data: File) -> Result<Self::Input, Error> {
        Ok(data.lines().map_results(|s| parse_line(&s)).try_collect()?)
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        let (safe_ingredients, _) = shared(&input);

        input
            .iter()
            .flat_map(|v| &v.0)
            .filter(|ing| safe_ingredients.contains(ing.as_str()))
            .count()
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        let (safe_ingredients, mut mappings) = shared(&input);

        for ingrs in mappings.values_mut() {
            *ingrs = &*ingrs - &safe_ingredients;
        }

        let mut list = Vec::new();
        while !mappings.is_empty() {
            for allergen in mappings.keys().cloned().collect_vec() {
                if let Some(ingredient) = mappings[&allergen].iter().exactly_one().ok().cloned() {
                    mappings.remove(&allergen);
                    for ingrs in mappings.values_mut() {
                        ingrs.remove(&ingredient);
                    }
                    list.push((allergen, ingredient));
                }
            }
        }

        list.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        list.into_iter().map(|(_, v)| v).join(",")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day21 as Challenge>::Input {
        [
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)",
            "trh fvjkl sbzzf mxmxvkd (contains dairy)",
            "sqjhc fvjkl (contains soy)",
            "sqjhc mxmxvkd sbzzf (contains fish)",
        ]
        .map(parse_line)
        .to_vec()
    }

    #[test]
    fn test_day21_part1() {
        assert_eq!(Day21::part1(sample_input()), 5);
    }

    #[test]
    fn test_day21_part2() {
        assert_eq!(Day21::part2(sample_input()), "mxmxvkd,sqjhc,fvjkl");
    }
}
