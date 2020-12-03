use crate::prelude::*;

pub enum Day03 {}

pub struct Row(Vec<bool>);

impl FromStr for Row {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v = Vec::with_capacity(s.len());
        for c in s.chars() {
            v.push(c == '#');
        }
        Ok(Self(v))
    }
}

fn traverse(trees: &[Row], dx: usize, dy: usize) -> usize {
    let mut n = 0;
    let (mut x, mut y) = (0, 0);
    while y < trees.len() {
        let row = &trees[y].0;
        if row[x % row.len()] {
            n += 1;
        }
        x += dx;
        y += dy;
    }
    n
}

impl Challenge for Day03 {
    type Input = Vec<Row>;
    type Output1 = usize;
    type Output2 = usize;

    fn read(data: File) -> Result<Self::Input, Error> {
        data.parse_lines()
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        traverse(&input, 3, 1)
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .copied()
            .map(|(dx, dy)| traverse(&input, dx, dy))
            .product()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_input() -> Vec<Row> {
        [
            "..##.......",
            "#...#...#..",
            ".#....#..#.",
            "..#.#...#.#",
            ".#...##..#.",
            "..#.##.....",
            ".#.#.#....#",
            ".#........#",
            "#.##...#...",
            "#...##....#",
            ".#..#...#.#",
        ]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect()
    }

    #[test]
    fn test_day02_part1() {
        assert_eq!(Day03::part1(test_input()), 7);
    }

    #[test]
    fn test_day02_part2() {
        assert_eq!(Day03::part2(test_input()), 336);
    }
}
