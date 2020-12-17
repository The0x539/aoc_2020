use crate::prelude::*;

use std::{collections::HashSet, convert::TryInto, ops::RangeInclusive};

type Point<const N: usize> = [i64; N];
type Volume<const N: usize> = [RangeInclusive<i64>; N];
type State<const N: usize> = HashSet<Point<N>>;

fn neighborhood<const N: usize>(point: Point<N>) -> Volume<N> {
    let v = point
        .iter()
        .copied()
        .map(|a| (a, a))
        .collect_vec()
        .try_into()
        .unwrap();
    volume(v)
}

fn volume<const N: usize>(minmaxes: [(i64, i64); N]) -> Volume<N> {
    minmaxes
        .iter()
        .copied()
        .map(|(a0, a1)| (a0 - 1)..=(a1 + 1))
        .collect_vec()
        .try_into()
        .unwrap()
}

fn iter_vol<const N: usize>(vol: Volume<N>) -> impl Iterator<Item = Point<N>> {
    vol.iter()
        .cloned()
        .multi_cartesian_product()
        .map(|p| p.try_into().unwrap())
}

fn next<const N: usize>(prev: &State<N>) -> State<N> {
    let minmax = |i| prev.iter().map(|p| p[i]).minmax().into_option().unwrap();
    let vol = volume((0..N).map(minmax).collect_vec().try_into().unwrap());

    let mut new = HashSet::new();
    for cell in iter_vol(vol) {
        let neighbor_count = iter_vol(neighborhood(cell))
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
        let mut state = parse(&input);
        for _ in 0..6 {
            state = next::<3>(&state);
        }
        state.len()
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        let mut state = parse(&input);
        for _ in 0..6 {
            state = next::<4>(&state);
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
