use crate::prelude::*;

use std::{collections::HashSet, ops::RangeInclusive};

type Point = Vec<i64>;
type Volume = Vec<RangeInclusive<i64>>;
type State = HashSet<Point>;

fn volume(minmaxes: Vec<(i64, i64)>) -> Volume {
    minmaxes
        .into_iter()
        .map(|(a0, a1)| (a0 - 1)..=(a1 + 1))
        .collect()
}

fn neighborhood(point: &Point) -> Volume {
    volume(point.iter().map(|&a| (a, a)).collect())
}

fn iter_vol(vol: Volume) -> impl Iterator<Item = Point> {
    vol.into_iter().multi_cartesian_product()
}

fn next(prev: &State, dims: usize) -> State {
    let minmax = |i| prev.iter().map(|p| p[i]).minmax().into_option().unwrap();
    let vol = volume((0..dims).map(minmax).collect());

    let mut new = HashSet::new();
    for cell in iter_vol(vol) {
        let neighbor_count = iter_vol(neighborhood(&cell))
            .filter(|p| p != &cell)
            .filter(|p| prev.contains(p))
            .count();

        if neighbor_count == 3 || (neighbor_count == 2 && prev.contains(&cell)) {
            new.insert(cell);
        }
    }
    new
}

fn parse(input: &[Vec<bool>], dims: usize) -> State {
    let mut state = HashSet::new();
    for (y, row) in input.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val {
                let mut point = vec![0; dims];
                point[0] = x as i64;
                point[1] = y as i64;
                state.insert(point);
            }
        }
    }
    state
}

pub enum Day17 {}

impl Challenge for Day17 {
    type Input = Vec<Vec<bool>>;
    type Output1 = usize;
    type Output2 = usize;

    fn read(data: File) -> Result<Self::Input, Error> {
        Ok(data
            .lines()
            .map_results(|s| s.bytes().map(|b| b == b'#').collect())
            .try_collect()?)
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        let mut state = parse(&input, 3);
        for _ in 0..6 {
            state = next(&state, 3);
        }
        state.len()
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        let mut state = parse(&input, 4);
        for _ in 0..6 {
            state = next(&state, 4);
        }
        state.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day17 as Challenge>::Input {
        vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
        ]
    }

    #[test]
    fn test_day17_part1() {
        assert_eq!(Day17::part1(sample_input()), 112);
    }

    #[test]
    fn test_day17_part2() {
        assert_eq!(Day17::part2(sample_input()), 848);
    }
}
