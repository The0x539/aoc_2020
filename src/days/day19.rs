use crate::prelude::*;

use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Char {
    A,
    B,
}

#[derive(Debug, PartialEq)]
pub enum Rule {
    Lit(Char),
    Seq(Vec<usize>),
    Alt(Vec<usize>, Vec<usize>),
}

impl FromStr for Rule {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "\"a\"" => return Ok(Rule::Lit(Char::A)),
            "\"b\"" => return Ok(Rule::Lit(Char::B)),
            _ => (),
        }

        if let Some(bar) = s.find('|') {
            let left = &s[..(bar - 1)];
            let right = &s[(bar + 2)..];
            let l = left.split(' ').map(|s| s.parse()).try_collect()?;
            let r = right.split(' ').map(|s| s.parse()).try_collect()?;
            Ok(Rule::Alt(l, r))
        } else {
            Ok(Rule::Seq(s.split(' ').map(|s| s.parse()).try_collect()?))
        }
    }
}

fn parse_rule(s: &str) -> Result<(usize, Rule), Error> {
    let colon = s.find(':').ok_or("no colon")?;
    let idx = s[..colon].parse()?;
    let rule = s[(colon + 2)..].parse()?;
    Ok((idx, rule))
}

#[allow(dead_code)]
fn expand_rule(rule: &Rule, rules: &HashMap<usize, Rule>) -> String {
    match rule {
        Rule::Lit(c) => match c {
            Char::A => "a".into(),
            Char::B => "b".into(),
        },
        Rule::Seq(seq) => seq.iter().map(|i| expand_rule(&rules[i], rules)).join(""),
        Rule::Alt(l, r) => format!(
            "({}|{})",
            expand_rule(&Rule::Seq(l.clone()), rules),
            expand_rule(&Rule::Seq(r.clone()), rules)
        ),
    }
}

fn match_rule<'a>(mut s: &'a str, rule: &Rule, rules: &HashMap<usize, Rule>) -> Option<&'a str> {
    match rule {
        Rule::Lit(Char::A) => s.strip_prefix('a'),
        Rule::Lit(Char::B) => s.strip_prefix('b'),
        Rule::Seq(seq) => {
            for idx in seq {
                s = match_rule(s, &rules[idx], rules)?;
            }
            Some(s)
        }
        Rule::Alt(l, r) => match_rule(s, &Rule::Seq(l.clone()), rules)
            .or_else(|| match_rule(s, &Rule::Seq(r.clone()), rules)),
        /*
        Rule::Alt(l, r) => match (&l[..], &r[..]) {
            ([a], [b, c]) if a == b && &rules[c] == rule => {
                let repeated_rule = &rules[a];
                s = match_rule(s, repeated_rule, rules)?; // must match at least one
                while let Some(new_s) = match_rule(s, repeated_rule, rules) {
                    s = new_s;
                }
                println!("rule 8 matched, yielding {}", s);
                Some(s)
            }
            ([a, b], [c, d, e]) if (a, b) == (c, e) && &rules[d] == rule => {
                let left_repeat = &rules[dbg!(a)];
                let right_repeat = &rules[b];
                println!("foo");
                s = match_rule(dbg!(s), left_repeat, rules)?; // must match at least one
                println!("bar");
                let mut n = 1;
                while let Some(new_s) = match_rule(s, left_repeat, rules) {
                    n += 1;
                    s = new_s;
                }
                println!("left half of rule 11 matched {} times", n);
                for _ in 0..n {
                    s = match_rule(s, right_repeat, rules)?;
                }
                println!("rule 11 matched");
                Some(s)
            }
            _ => match_rule(s, &Rule::Seq(l.clone()), rules)
                .or_else(|| match_rule(s, &Rule::Seq(r.clone()), rules)),
        },
        */
    }
}

fn match_suffix<'a>(mut s: &'a str, rule: &Rule, rules: &HashMap<usize, Rule>) -> Option<&'a str> {
    match rule {
        Rule::Lit(Char::A) => s.strip_suffix('a'),
        Rule::Lit(Char::B) => s.strip_suffix('b'),
        Rule::Seq(seq) => {
            for idx in seq.iter().rev() {
                s = match_suffix(s, &rules[idx], rules)?;
            }
            Some(s)
        }
        Rule::Alt(l, r) => match_suffix(s, &Rule::Seq(l.clone()), rules)
            .or_else(|| match_suffix(s, &Rule::Seq(r.clone()), rules)),
    }
}

fn match_part2(mut s: &str, rules: &HashMap<usize, Rule>) -> bool {
    // rule 11 requires at least one 42+31 pair
    s = match match_suffix(s, &rules[&31], rules) {
        Some(new_s) => new_s,
        None => return false,
    };

    let mut num_pairs = 1;
    // right half of rule 11
    while let Some(new_s) = match_suffix(s, &rules[&31], rules) {
        s = new_s;
        num_pairs += 1;
    }
    // left half of rule 11
    for _ in 0..num_pairs {
        if let Some(new_s) = match_suffix(s, &rules[&42], rules) {
            s = new_s;
        }
    }

    // rule 8 requires at least one standalone 42
    s = match match_rule(s, &rules[&42], rules) {
        Some(new_s) => new_s,
        None => return false,
    };

    // rule 8
    while let Some(new_s) = match_rule(s, &rules[&42], rules) {
        s = new_s;
    }
    s == ""
}

pub enum Day19 {}

impl Challenge for Day19 {
    type Input = (HashMap<usize, Rule>, Vec<String>);
    type Output1 = usize;
    type Output2 = usize;

    fn read(data: File) -> Result<Self::Input, Error> {
        let mut lines = data.lines();
        let mut rules = HashMap::new();
        for line in &mut lines {
            let line = line?;
            if line == "" {
                break;
            }
            let (idx, rule) = parse_rule(&line)?;
            rules.insert(idx, rule);
        }
        let msgs = lines.try_collect()?;
        Ok((rules, msgs))
    }

    fn part1((rules, msgs): Self::Input) -> Self::Output1 {
        msgs.into_iter()
            .filter(|msg| match_rule(&msg, &rules[&0], &rules) == Some(""))
            .count()
    }

    fn part2((rules, msgs): Self::Input) -> Self::Output2 {
        let mut count = 0;
        for msg in msgs {
            if match_part2(&msg, &rules) {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day19 as Challenge>::Input {
        let rules = vec![
            "0: 4 1 5",
            "1: 2 3 | 3 2",
            "2: 4 4 | 5 5",
            "3: 4 5 | 5 4",
            "4: \"a\"",
            "5: \"b\"",
        ]
        .into_iter()
        .map(parse_rule)
        .try_collect()
        .unwrap();

        let msgs = ["ababbb", "bababa", "abbbab", "aaabbb", "aaaabbb"]
            .map(String::from)
            .to_vec();

        (rules, msgs)
    }

    fn sample_input2() -> <Day19 as Challenge>::Input {
        let rules = vec![
            "42: 9 14 | 10 1",
            "9: 14 27 | 1 26",
            "10: 23 14 | 28 1",
            "1: \"a\"",
            "11: 42 31",
            "5: 1 14 | 15 1",
            "19: 14 1 | 14 14",
            "12: 24 14 | 19 1",
            "16: 15 1 | 14 14",
            "31: 14 17 | 1 13",
            "6: 14 14 | 1 14",
            "2: 1 24 | 14 4",
            "0: 8 11",
            "13: 14 3 | 1 12",
            "15: 1 | 14",
            "17: 14 2 | 1 7",
            "23: 25 1 | 22 14",
            "28: 16 1",
            "4: 1 1",
            "20: 14 14 | 1 15",
            "3: 5 14 | 16 1",
            "27: 1 6 | 14 18",
            "14: \"b\"",
            "21: 14 1 | 1 14",
            "25: 1 1 | 1 14",
            "22: 14 14",
            "8: 42",
            "26: 14 22 | 1 20",
            "18: 15 15",
            "7: 14 5 | 1 21",
            "24: 14 1",
        ]
        .into_iter()
        .map(parse_rule)
        .try_collect()
        .unwrap();

        let msgs = [
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ]
        .map(String::from)
        .to_vec();

        (rules, msgs)
    }

    #[test]
    fn test_day19_part1() {
        assert_eq!(Day19::part1(sample_input()), 2);
    }

    #[test]
    fn test_day19_part2() {
        assert_eq!(Day19::part2(sample_input2()), 12);
    }
}
