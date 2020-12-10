use crate::prelude::*;

pub enum Day09 {}

impl Challenge for Day09 {
    type Input = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    fn read(data: File) -> Result<Self::Input, Error> {
        data.parse_lines()
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        'test: for i in 25..input.len() {
            let predecessors = (i - 25)..i;
            for (j, k) in predecessors.tuple_combinations() {
                if input[j] + input[k] == input[i] {
                    continue 'test;
                }
            }
            return input[i];
        }
        panic!("all numbers seem valid")
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        let target = Self::part1(input.clone());
        let mut range = 0..0;

        macro_rules! sum {
            (.) => {
                input[range.clone()].iter().copied()
            };
            () => {
                sum!(.).sum::<usize>()
            };
        }

        loop {
            while sum!() < target {
                range.end += 1;
            }
            while sum!() > target {
                range.start += 1;
            }
            if sum!() == target {
                break;
            }
        }

        sum!(.).min().unwrap() + sum!(.).max().unwrap()
    }
}

/*
#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day09 as Challenge>::Input {
        vec![]
    }

    #[test]
    fn test_day09_part1() {
        assert_eq!(Day09::part1(sample_input()), ());
    }

    #[test]
    fn test_day09_part2() {
        assert_eq!(Day09::part2(sample_input()), ());
    }
}
*/
