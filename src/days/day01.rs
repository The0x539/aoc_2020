use std::io::BufRead;

use itertools::Itertools;

pub enum Day01 {}

fn find_2020_pair(nums: &[u32]) -> (u32, u32) {
    nums.iter()
        .copied()
        .tuple_combinations()
        .find(|(a, b)| a + b == 2020)
        .unwrap()
}

fn find_2020_triplet(nums: &[u32]) -> (u32, u32, u32) {
    nums.iter()
        .copied()
        .tuple_combinations()
        .find(|(a, b, c)| a + b + c == 2020)
        .unwrap()
}

impl crate::Challenge for Day01 {
    type Input = Vec<u32>;
    type Output1 = u32;
    type Output2 = u32;

    fn read(data: std::io::BufReader<std::fs::File>) -> Self::Input {
        data.lines().map(|s| s.unwrap().parse().unwrap()).collect()
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        let (a, b) = find_2020_pair(&input);
        a * b
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        let (a, b, c) = find_2020_triplet(&input);
        a * b * c
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_day01_part1() {
        let sample = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(super::find_2020_pair(&sample), (1721, 299));
    }

    #[test]
    fn test_day01_part2() {
        let sample = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(super::find_2020_triplet(&sample), (979, 366, 675));
    }
}
