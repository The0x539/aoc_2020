use crate::prelude::*;

use itertools::MinMaxResult;
use std::{collections::HashSet, convert::TryInto, ops::RangeInclusive};

type Point<const N: usize> = [i8; N];
type Volume<const N: usize> = [RangeInclusive<i8>; N];
type State<const N: usize> = HashSet<Point<N>>;

fn neighborhood<const N: usize>(point: &Point<N>) -> Volume<N> {
    point.map(|a| (a - 1)..=(a + 1))
}

fn iter_vol<const N: usize>(vol: Volume<N>) -> impl Iterator<Item = Point<N>> {
    vol.iter()
        .cloned()
        .multi_cartesian_product()
        .map(|p| p.try_into().unwrap())
}

fn next<const N: usize>(prev: &State<N>) -> State<N> {
    let minmax = |i| prev.iter().map(|p| p[i]).minmax();
    let mut vol = [0; N].map(|_| 0..=0); // stupid non-Copy ranges and non-Default arrays
    for i in 0..N {
        vol[i] = match minmax(i) {
            MinMaxResult::MinMax(min, max) => (min - 1)..=(max + 1),
            // if there aren't at least 3 live cells, next gen has no life
            _ => return HashSet::new(),
        };
    }

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

fn parse<const N: usize>(input: &[Vec<bool>]) -> State<N> {
    let mut state = HashSet::new();
    for (y, row) in input.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val {
                let mut point = [0; N];
                point[0] = x as _;
                point[1] = y as _;
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
        let mut state: State<3> = parse(&input);
        for _ in 0..6 {
            state = next(&state);
        }
        state.len()
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        let mut state: State<4> = parse(&input);
        for _ in 0..6 {
            state = next(&state);
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
