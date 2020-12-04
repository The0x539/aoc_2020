use crate::prelude::*;

use regex::Regex;
use std::collections::HashMap;

pub enum Day04 {}

fn valid(passport: &HashMap<String, String>) -> bool {
    let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for field in &required_fields {
        if !passport.contains_key(*field) {
            return false;
        }
    }
    true
}

fn v2(passport: &HashMap<String, String>) -> Option<()> {
    macro_rules! check {
        ($cond:expr) => {
            if !($cond) {
                return None;
            }
        };
    }

    let byr = passport.get("byr")?;
    check!(byr.len() == 4);
    let byr = byr.parse::<u16>().ok()?;
    check!((1920..=2002).contains(&byr));

    let iyr = passport.get("iyr")?;
    check!(iyr.len() == 4);
    let iyr = iyr.parse::<u16>().ok()?;
    check!((2010..=2020).contains(&iyr));

    let eyr = passport.get("eyr")?;
    check!(eyr.len() == 4);
    let eyr = eyr.parse::<u16>().ok()?;
    check!((2020..=2030).contains(&eyr));

    let hgt = passport.get("hgt")?;
    if hgt.ends_with("cm") {
        check!(hgt.len() == 5);
        let hgt = hgt[0..3].parse::<u8>().ok()?;
        check!((150..=193).contains(&hgt));
    } else if hgt.ends_with("in") {
        check!(hgt.len() == 4);
        let hgt = hgt[0..2].parse::<u8>().ok()?;
        check!((59..=76).contains(&hgt));
    } else {
        return None;
    };

    let hcl = passport.get("hcl")?;
    check!(hcl.len() == 7);
    check!(hcl.nth_char(0) == '#');
    check!(hcl[1..].chars().all(|c| c.is_ascii_hexdigit()));

    let ecl = passport.get("ecl")?;
    check!(matches!(
        ecl.as_str(),
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
    ));

    let pid = passport.get("pid")?;
    check!(pid.len() == 9);
    check!(pid.chars().all(|c| c.is_ascii_digit()));

    Some(())
}

fn valid2(passport: &HashMap<String, String>) -> bool {
    v2(passport).is_some()
}

impl Challenge for Day04 {
    type Input = Vec<HashMap<String, String>>;
    type Output1 = usize;
    type Output2 = usize;

    fn read(data: File) -> Result<Self::Input, Error> {
        let mut passports = Vec::new();
        let mut cur_passport = HashMap::new();
        let re = Regex::new(r"(\S+):(\S+)").unwrap();
        for line in data.lines() {
            let line = line?;
            if line.is_empty() {
                passports.push(cur_passport);
                cur_passport = HashMap::new();
            } else {
                for caps in re.captures_iter(&line) {
                    cur_passport.insert(caps[1].into(), caps[2].into());
                }
            }
        }
        Ok(passports)
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        input.iter().filter(|x| valid(x)).count()
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        input.iter().filter(|x| valid2(x)).count()
    }
}
