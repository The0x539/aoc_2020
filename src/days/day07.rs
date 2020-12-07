use crate::prelude::*;

use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
pub struct Color(String, String);

impl Color {
    fn new(a: impl Into<String>, b: impl Into<String>) -> Self {
        Self(a.into(), b.into())
    }

    fn from_iter<T>(iter: &mut T) -> Option<Self>
    where
        T: Iterator,
        T::Item: Into<String>,
    {
        Some(Self::new(iter.next()?, iter.next()?))
    }
}

type Rule = (Color, Vec<SomeBags>);

#[derive(Debug, Clone)]
pub struct SomeBags {
    amount: usize,
    color: Color,
}

fn parse_rule(s: &str) -> Result<Rule, Error> {
    let mut words = s.split(' ');
    let color = Color::from_iter(&mut words).unwrap();
    words.next(); // "bags"
    words.next(); // "contain"
    let mut contents = Vec::new();
    while let Some(n) = words.next() {
        if n == "no" {
            break;
        }
        contents.push(SomeBags {
            amount: n.parse()?,
            color: Color::from_iter(&mut words).unwrap(),
        });
        words.next(); // "bags"
    }
    Ok((color, contents))
}

pub enum Day07 {}

impl Challenge for Day07 {
    type Input = HashMap<Color, Vec<SomeBags>>;
    type Output1 = usize;
    type Output2 = usize;

    fn read(data: File) -> Result<Self::Input, Error> {
        let mut rules = HashMap::new();
        for line in data.lines() {
            let line = line?;
            let (parent, children) = parse_rule(&line)?;
            rules.insert(parent, children);
        }
        Ok(rules)
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        let mut ps = HashSet::new();
        ps.insert(Color::new("shiny", "gold"));
        loop {
            let mut should_break = true;
            for (color, contents) in &input {
                if ps.contains(color) {
                    continue;
                }

                for child in contents {
                    if ps.contains(&child.color) {
                        ps.insert(color.clone());
                        should_break = false;
                        break;
                    }
                }
            }
            if should_break {
                break;
            }
        }
        ps.len() - 1
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        let mut n = 0;
        let mut bags = vec![SomeBags {
            amount: 1,
            color: Color::new("shiny", "gold"),
        }];

        while let Some(bag) = bags.pop() {
            n += bag.amount;
            for child in &input[&bag.color] {
                let multiplied_child = SomeBags {
                    amount: bag.amount * child.amount,
                    color: child.color.clone(),
                };
                bags.push(multiplied_child);
            }
        }

        n - 1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day07 as Challenge>::Input {
        let s = [
            "light red bags contain 1 bright white bag, 2 muted yellow bags.",
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
            "bright white bags contain 1 shiny gold bag.",
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
            "faded blue bags contain no other bags.",
            "dotted black bags contain no other bags.",
        ];
        s.iter().map(|s| parse_rule(*s).unwrap()).collect()
    }

    fn sample_input2() -> <Day07 as Challenge>::Input {
        let s = [
            "shiny gold bags contain 2 dark red bags.",
            "dark red bags contain 2 dark orange bags.",
            "dark orange bags contain 2 dark yellow bags.",
            "dark yellow bags contain 2 dark green bags.",
            "dark green bags contain 2 dark blue bags.",
            "dark blue bags contain 2 dark violet bags.",
            "dark violet bags contain no other bags.",
        ];
        s.iter().map(|s| parse_rule(*s).unwrap()).collect()
    }

    #[test]
    fn test_day07_part1() {
        assert_eq!(Day07::part1(sample_input()), 4);
    }

    #[test]
    fn test_day07_part2() {
        assert_eq!(Day07::part2(sample_input()), 32);
        assert_eq!(Day07::part2(sample_input2()), 126);
    }
}
