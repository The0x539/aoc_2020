use crate::prelude::*;

pub enum Day10 {}

impl Challenge for Day10 {
    type Input = Vec<u32>;
    type Output1 = u32;
    type Output2 = u64;

    fn read(data: File) -> Result<Self::Input, Error> {
        data.parse_lines()
    }

    fn part1(mut input: Self::Input) -> Self::Output1 {
        input.sort();
        input.insert(0, 0); // sorry
        input.push(input.last().unwrap() + 3);

        let mut diff1 = 0;
        let mut diff3 = 0;
        for (a, b) in input.into_iter().tuple_windows() {
            match b - a {
                1 => diff1 += 1,
                3 => diff3 += 1,
                _ => (),
            }
        }

        diff1 * diff3
    }

    fn part2(mut input: Self::Input) -> Self::Output2 {
        input.push(0);
        input.sort();

        // THIS IS THE SNEAKY PART
        assert!(input.iter().tuple_windows().all(|(a, b)| b - a != 2));

        let lookup = [
            // can't omit anything
            1, 1, 1, // can omit b from abc or can choose not to
            2, // can omit nothing, b, c, or bc from abcd
            4,
            // can omit any combination except bcd from abcde
            // (because then there'd be a joltage gap of 4)
            7,
            // further group sizes unhandled; would panic
        ];

        let mut n = 1;

        let mut group_size = 1;
        for i in 1..input.len() {
            if input[i] - input[i - 1] == 3 {
                n *= lookup[group_size];
                group_size = 0;
            }
            group_size += 1;
        }
        n *= lookup[group_size]; // the final group

        n
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day10 as Challenge>::Input {
        vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
    }

    fn sample_input2() -> <Day10 as Challenge>::Input {
        vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ]
    }

    #[test]
    fn test_day10_part1() {
        assert_eq!(Day10::part1(sample_input()), 7 * 5);
        assert_eq!(Day10::part1(sample_input2()), 22 * 10);
    }

    #[test]
    fn test_day10_part2() {
        assert_eq!(Day10::part2(sample_input()), 8);
        assert_eq!(Day10::part2(sample_input2()), 19208);
    }
}
