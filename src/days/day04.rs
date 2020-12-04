use crate::prelude::*;

use regex::Regex;
use std::collections::HashMap;

pub enum Day04 {}

fn valid(passport: &HashMap<String, String>) -> bool {
    for field in &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
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

    // birth year
    let byr: u16 = passport.get("byr")?.parse().ok()?;
    check!((1920..=2002).contains(&byr));

    // issue year
    let iyr: u16 = passport.get("iyr")?.parse().ok()?;
    check!((2010..=2020).contains(&iyr));

    // expiry year
    let eyr: u16 = passport.get("eyr")?.parse().ok()?;
    check!((2020..=2030).contains(&eyr));

    // height
    let hgt = passport.get("hgt")?;
    if hgt.ends_with("cm") {
        check!(hgt.len() == 5);
        let hgt: u8 = hgt[..3].parse().ok()?;
        check!((150..=193).contains(&hgt));
    } else if hgt.ends_with("in") {
        check!(hgt.len() == 4);
        let hgt: u8 = hgt[..2].parse().ok()?;
        check!((59..=76).contains(&hgt));
    } else {
        return None;
    };

    // hair color
    let hcl = passport.get("hcl")?;
    check!(hcl.len() == 7);
    check!(hcl.nth_char(0) == '#');
    check!(hcl[1..].chars().all(|c| c.is_ascii_hexdigit()));

    // eye color
    let ecl = passport.get("ecl")?.as_str();
    if !matches!(ecl, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth") {
        return None;
    }

    // passport id
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
