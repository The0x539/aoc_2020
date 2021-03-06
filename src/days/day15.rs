use crate::prelude::*;

use std::collections::HashMap;

pub enum Day15 {}

fn play_until(end: u64, input: &[u64]) -> u64 {
    let mut t = 0;
    let mut last_spoken = 0;
    let mut foo = HashMap::new();
    for n in input.iter().copied() {
        foo.insert(n, t);
        last_spoken = n;
        t += 1;
    }
    while t < end {
        let bar = foo.entry(last_spoken).or_insert(t - 1);
        let age = (t - 1) - *bar;
        *bar = t - 1;
        last_spoken = age;
        t += 1;
    }
    last_spoken
}

impl Challenge for Day15 {
    type Input = Vec<u64>;
    type Output1 = u64;
    type Output2 = u64;

    fn read(mut data: File) -> Result<Self::Input, Error> {
        let mut s = String::new();
        data.read_to_string(&mut s)?;
        s.split(',').map(|s| Ok(s.trim().parse()?)).collect()
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        play_until(2020, &input)
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        play_until(30000000, &input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> Vec<<Day15 as Challenge>::Input> {
        vec![
            vec![0, 3, 6],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![1, 2, 3],
            vec![2, 3, 1],
            vec![3, 2, 1],
            vec![3, 1, 2],
        ]
    }

    #[test]
    fn test_day15_part1() {
        assert_eq!(
            sample_input().into_iter().map(Day15::part1).collect_vec(),
            [436, 1, 10, 27, 78, 438, 1836],
        );
    }

    /*
    #[test]
    fn test_day15_part2() {
        assert_eq!(
            sample_input().into_iter().map(Day15::part2).collect_vec(),
            [175594, 2578, 3544142, 261214, 6895259, 18, 362],
        );
    }
    */
}
