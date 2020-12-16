use crate::prelude::*;

use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
};

#[derive(Debug, Clone)]
struct Range(RangeInclusive<usize>, RangeInclusive<usize>);

fn parse_range(s: &str) -> Result<RangeInclusive<usize>, Error> {
    let dash = s.find('-').ok_or("no dash")?;
    let a = s[..dash].parse()?;
    let b = s[(dash + 1)..].parse()?;
    Ok(a..=b)
}

fn parse_ticket(s: &str) -> Result<Ticket, Error> {
    Ok(s.split(',').map(|v| v.parse()).try_collect()?)
}

type Ticket = Vec<usize>;

pub struct Input {
    fields: HashMap<String, Range>,
    my_ticket: Ticket,
    tickets: Vec<Ticket>,
}

impl Range {
    fn contains(&self, n: usize) -> bool {
        self.0.contains(&n) || self.1.contains(&n)
    }
}

fn invalid(val: usize, fields: &HashMap<String, Range>) -> bool {
    !fields.values().any(|range| range.contains(val))
}

fn determine_fields(fields: &HashMap<String, Range>, tickets: &[Ticket]) -> HashMap<String, usize> {
    let n_fields = fields.len();

    let mut mappings: HashMap<&String, HashSet<usize>> = fields
        .keys()
        .map(|k| (k, (0..n_fields).collect()))
        .collect();

    for ticket in tickets {
        if ticket.iter().any(|val| invalid(*val, fields)) {
            // ticket is completely invalid
            continue;
        }

        for (i, val) in ticket.iter().enumerate() {
            for (name, map) in &mut mappings {
                if !fields[*name].contains(*val) {
                    map.remove(&i);
                }
            }
        }
    }

    loop {
        let mut changed = false;

        for name in fields.keys() {
            if let Ok(&i) = mappings[name].iter().exactly_one() {
                for (n, map) in &mut mappings {
                    if n != &name {
                        changed |= map.remove(&i);
                    }
                }
            }
        }

        if !changed {
            break;
        }
    }

    assert!(mappings.values().all(|m| m.len() == 1));

    mappings
        .into_iter()
        .map(|(k, v)| (k.clone(), v.into_iter().exactly_one().unwrap()))
        .collect()
}

pub enum Day16 {}

impl Challenge for Day16 {
    type Input = Input;
    type Output1 = usize;
    type Output2 = usize;

    fn read(data: File) -> Result<Self::Input, Error> {
        let mut lines = data.lines();

        let mut fields = HashMap::new();
        for line in lines.by_ref() {
            let line = line?;
            if line == "" {
                break;
            }
            let colon = line.find(':').ok_or("no colon")?;
            let field = line[..colon].into();
            let (r1, _, r2) = line[(colon + 2)..]
                .split(' ')
                .collect_tuple()
                .ok_or("bad rule fmt")?;
            let r1 = parse_range(r1)?;
            let r2 = parse_range(r2)?;
            fields.insert(field, Range(r1, r2));
        }

        lines.next(); // "your ticket:"
        let my_ticket = parse_ticket(lines.next().ok_or("expected my ticket")??.as_str())?;

        lines.next();
        lines.next(); // "nearby tickets:"

        let tickets = lines.map(|s| parse_ticket(s?.as_str())).try_collect()?;

        Ok(Input {
            fields,
            my_ticket,
            tickets,
        })
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        let Input {
            fields, tickets, ..
        } = input;

        tickets
            .iter()
            .flatten()
            .filter(|val| invalid(**val, &fields))
            .sum()
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        let Input {
            fields,
            my_ticket,
            tickets,
        } = input;

        determine_fields(&fields, &tickets)
            .into_iter()
            .filter(|(k, _)| k.starts_with("departure"))
            .map(|(_, v)| my_ticket[v])
            .product()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day16 as Challenge>::Input {
        let mut fields = HashMap::new();
        fields.insert("class".into(), Range(1..=3, 5..=7));
        fields.insert("row".into(), Range(6..=11, 33..=44));
        fields.insert("seat".into(), Range(13..=40, 45..=50));

        Input {
            fields,
            my_ticket: vec![7, 1, 14],
            tickets: vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12],
            ],
        }
    }

    #[test]
    fn test_day16_part1() {
        assert_eq!(Day16::part1(sample_input()), 71);
    }

    #[test]
    fn test_day16_part2() {
        let expected = ["row", "class", "seat"]
            .iter()
            .enumerate()
            .map(|(v, k)| (k.to_string(), v))
            .collect();

        let input = sample_input();
        assert_eq!(determine_fields(&input.fields, &input.tickets), expected);
    }
}
