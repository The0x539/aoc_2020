use crate::prelude::*;

use itertools::iproduct;
use std::{collections::HashSet, ops::RangeInclusive};

type Point = [i64; 3];
type Point4 = [i64; 4];
type State = HashSet<Point>;
type State4 = HashSet<Point4>;

fn volume(xyz: [(i64, i64); 3]) -> [RangeInclusive<i64>; 3] {
    let [(x0, x1), (y0, y1), (z0, z1)] = xyz;
    [
        (x0 - 1)..=(x1 + 1),
        (y0 - 1)..=(y1 + 1),
        (z0 - 1)..=(z1 + 1),
    ]
}

fn volume4(xyzw: [(i64, i64); 4]) -> [RangeInclusive<i64>; 4] {
    let [(x0, x1), (y0, y1), (z0, z1), (w0, w1)] = xyzw;
    [
        (x0 - 1)..=(x1 + 1),
        (y0 - 1)..=(y1 + 1),
        (z0 - 1)..=(z1 + 1),
        (w0 - 1)..=(w1 + 1),
    ]
}

fn iter_vol(vol: [RangeInclusive<i64>; 3]) -> impl Iterator<Item = Point> {
    let [vx, vy, vz] = vol;
    iproduct!(vx, vy, vz).map(|(x, y, z)| [x, y, z])
}

fn iter_vol4(vol: [RangeInclusive<i64>; 4]) -> impl Iterator<Item = Point4> {
    let [vx, vy, vz, vw] = vol;
    iproduct!(vx, vy, vz, vw).map(|(x, y, z, w)| [x, y, z, w])
}

fn next(prev: &State) -> State {
    let minmax = |i| prev.iter().map(|p| p[i]).minmax().into_option().unwrap();
    let vol = volume([minmax(0), minmax(1), minmax(2)]);

    let mut new = HashSet::new();
    for [x, y, z] in iter_vol(vol) {
        let neighborhood = volume([(x, x), (y, y), (z, z)]);
        let neighbor_count = iter_vol(neighborhood)
            .filter(|p| p != &[x, y, z])
            .filter(|p| prev.contains(p))
            .count();

        if neighbor_count == 3 || (neighbor_count == 2 && prev.contains(&[x, y, z])) {
            new.insert([x, y, z]);
        }
    }
    new
}

fn next4(prev: &State4) -> State4 {
    let minmax = |i| prev.iter().map(|p| p[i]).minmax().into_option().unwrap();
    let vol = volume4([minmax(0), minmax(1), minmax(2), minmax(3)]);

    let mut new = HashSet::new();
    for [x, y, z, w] in iter_vol4(vol) {
        let neighborhood = volume4([(x, x), (y, y), (z, z), (w, w)]);
        let neighbor_count = iter_vol4(neighborhood)
            .filter(|p| p != &[x, y, z, w])
            .filter(|p| prev.contains(p))
            .count();

        if neighbor_count == 3 || (neighbor_count == 2 && prev.contains(&[x, y, z, w])) {
            new.insert([x, y, z, w]);
        }
    }
    new
}

fn parse(input: &[Vec<bool>]) -> State {
    let mut state = HashSet::new();
    for (y, row) in input.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val {
                state.insert([x as _, y as _, 0]);
            }
        }
    }
    state
}

fn parse4(input: &[Vec<bool>]) -> State4 {
    let mut state = HashSet::new();
    for (y, row) in input.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val {
                state.insert([x as _, y as _, 0, 0]);
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
            state = next(&state);
        }
        state.len()
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        let mut state = parse4(&input);
        for _ in 0..6 {
            state = next4(&state);
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
