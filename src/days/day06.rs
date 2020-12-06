use crate::prelude::*;

use std::collections::HashSet;

pub enum Day06 {}

impl Challenge for Day06 {
    type Input = Vec<Vec<HashSet<char>>>;
    type Output1 = usize;
    type Output2 = usize;

    fn read(data: File) -> Result<Self::Input, Error> {
        let mut all = Vec::new();
        let mut group = Vec::new();
        for line in data.lines() {
            let line = line?;
            if line.is_empty() {
                all.push(group);
                group = Vec::new();
            } else {
                group.push(line.chars().collect());
            }
        }
        all.push(group);
        Ok(all)
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        input
            .into_iter()
            .map(|group| group.into_iter().flatten().collect::<HashSet<_>>().len())
            .sum()
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        input
            .into_iter()
            .map(|group| group.into_iter().fold1(|a, b| &a & &b).unwrap().len())
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day06 as Challenge>::Input {
        let x: &[&[&str]] = &[
            &["abc"],
            &["a", "b", "c"],
            &["ab", "ac"],
            &["a", "a", "a", "a"],
            &["b"],
        ];
        x.iter()
            .map(|g| g.iter().map(|m| m.chars().collect()).collect())
            .collect()
    }

    #[test]
    fn test_day06_part1() {
        assert_eq!(Day06::part1(sample_input()), 11);
    }

    #[test]
    fn test_day06_part2() {
        assert_eq!(Day06::part2(sample_input()), 6);
    }
}
