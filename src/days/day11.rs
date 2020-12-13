use crate::prelude::*;

use std::ops::Add;

use itertools::iproduct;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    Empty = b'L',
    Occupied = b'#',
    Floor = b'.',
}

impl State {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => panic!(),
        }
    }
}

fn generation(prev: &[Vec<State>]) -> (Vec<Vec<State>>, bool) {
    let mut did_change = false;
    let mut new = prev.to_owned();
    for (x, y) in iproduct!(0..prev[0].len(), 0..prev.len()) {
        let seat = &mut new[y][x];
        if *seat == State::Floor {
            continue;
        }

        let (min_x, min_y) = (x.saturating_sub(1), y.saturating_sub(1));
        let (max_x, max_y) = (
            x.add(1).min(prev[0].len() - 1),
            y.add(1).min(prev.len() - 1),
        );

        let num_cells = iproduct!(min_x..=max_x, min_y..=max_y)
            .map(|(x, y)| (prev[y][x] == State::Occupied) as u8)
            .sum::<u8>();

        if *seat == State::Empty && num_cells == 0 {
            *seat = State::Occupied;
            did_change = true;
        } else if *seat == State::Occupied && num_cells >= 5 {
            *seat = State::Empty;
            did_change = true;
        }
    }
    (new, did_change)
}

fn gen2(prev: &[Vec<State>]) -> (Vec<Vec<State>>, bool) {
    let mut did_change = false;
    let mut new = prev.to_owned();
    for (x, y) in iproduct!(0..prev[0].len(), 0..prev.len()) {
        let seat = &mut new[y][x];
        if *seat == State::Floor {
            continue;
        }

        let mut num_cells = 0;
        for (dx, dy) in iproduct!(-1..=1, -1..=1).filter(|v| *v != (0, 0)) {
            for r in 1.. {
                if (r > x && dx < 0) || (r + x >= prev[0].len() && dx > 0) {
                    break;
                } else if (r > y && dy < 0) || (r + y >= prev.len() && dy > 0) {
                    break;
                }
                let i = x as isize + r as isize * dx;
                let j = y as isize + r as isize * dy;

                let s = prev[j as usize][i as usize];
                if s == State::Occupied {
                    num_cells += 1;
                    break;
                } else if s == State::Empty {
                    break;
                }
            }
        }

        if *seat == State::Empty && num_cells == 0 {
            *seat = State::Occupied;
            did_change = true;
        } else if *seat == State::Occupied && num_cells >= 5 {
            *seat = State::Empty;
            did_change = true;
        }
    }
    (new, did_change)
}

pub enum Day11 {}

impl Challenge for Day11 {
    type Input = Vec<Vec<State>>;
    type Output1 = usize;
    type Output2 = usize;

    fn read(data: File) -> Result<Self::Input, Error> {
        let mut rows = Vec::new();
        for line in data.lines() {
            rows.push(line?.chars().map(State::from_char).collect());
        }
        Ok(rows)
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        let mut gen = input;
        loop {
            let (new_gen, did_change) = generation(&gen);
            gen = new_gen;
            if !did_change {
                break;
            }
        }
        gen.into_iter()
            .flatten()
            .filter(|x| *x == State::Occupied)
            .count()
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        let mut gen = input;
        loop {
            let (new_gen, did_change) = gen2(&gen);
            gen = new_gen;

            /*
            for row in &gen {
                for seat in row {
                    print!("{}", *seat as u8 as char);
                }
                println!();
            }
            println!();
            */

            if !did_change {
                break;
            }
        }
        gen.into_iter()
            .flatten()
            .filter(|x| *x == State::Occupied)
            .count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day11 as Challenge>::Input {
        [
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]
        .iter()
        .copied()
        .map(|line| line.chars().map(State::from_char).collect())
        .collect()
    }

    #[test]
    fn test_day11_part1() {
        assert_eq!(Day11::part1(sample_input()), 37);
    }

    #[test]
    fn test_day11_part2() {
        assert_eq!(Day11::part2(sample_input()), 26);
    }
}
