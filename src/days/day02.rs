use crate::prelude::*;

use regex::Regex;
use std::ops::RangeInclusive;

#[derive(Debug)]
pub struct Line {
    amt: RangeInclusive<usize>,
    c: char,
    password: String,
}

impl Line {
    const fn new(amt: RangeInclusive<usize>, c: char, password: String) -> Self {
        Self { amt, c, password }
    }

    fn is_valid(&self) -> bool {
        let count = self.password.chars().filter(|c| *c == self.c).count();
        self.amt.contains(&count)
    }

    fn is_valid_p2(&self) -> bool {
        let i1 = self.amt.start() - 1;
        let i2 = self.amt.end() - 1;
        let a = self.password.nth_char(i1);
        let b = self.password.nth_char(i2);
        (a == self.c) != (b == self.c)
    }
}

impl FromStr for Line {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([0-9]+)\-([0-9]+) (.): ([a-z]+)").unwrap();
        let caps = re.try_captures(s)?;
        let lo = caps[1].parse()?;
        let hi = caps[2].parse()?;
        let c = caps[3].nth_char(0);
        let password = caps[4].into();
        Ok(Self::new(lo..=hi, c, password))
    }
}

pub enum Day02 {}

impl Challenge for Day02 {
    type Input = Vec<Line>;
    type Output1 = usize;
    type Output2 = usize;

    fn read(data: File) -> Result<Self::Input, Error> {
        data.parse_lines()
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        input.into_iter().filter(Line::is_valid).count()
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        input.into_iter().filter(Line::is_valid_p2).count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> [Line; 3] {
        [
            Line::new(1..=3, 'a', "abcde".into()),
            Line::new(1..=3, 'b', "cdefg".into()),
            Line::new(2..=9, 'c', "ccccccccc".into()),
        ]
    }

    #[test]
    fn test_day02_part1() {
        assert_eq!(
            test_input().iter().map(Line::is_valid).collect_vec(),
            [true, false, true]
        );
    }

    #[test]
    fn test_day02_part2() {
        assert_eq!(
            test_input().iter().map(Line::is_valid_p2).collect_vec(),
            [true, false, false]
        );
    }
}
