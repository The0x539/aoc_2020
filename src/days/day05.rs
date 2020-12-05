use crate::prelude::*;

use std::collections::HashSet;

pub enum Day05 {}

fn seat_pos(seat: &str) -> (u8, u8) {
    let mut row = 0;
    for c in seat[..7].chars() {
        row <<= 1;
        if c == 'B' {
            row |= 1;
        }
    }

    let mut col = 0;
    for c in seat[7..].chars() {
        col <<= 1;
        if c == 'R' {
            col |= 1;
        }
    }

    (row, col)
}

fn seat_id(pos: (u8, u8)) -> u16 {
    let (row, col) = (pos.0 as u16, pos.1 as u16);
    row * 8 + col
}

impl Challenge for Day05 {
    type Input = Vec<String>;
    type Output1 = u16;
    type Output2 = u16;

    fn read(data: File) -> Result<Self::Input, Error> {
        Ok(data.lines().try_collect()?)
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        input.iter().map(|s| seat_id(seat_pos(s))).max().unwrap()
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        let seats: HashSet<_> = input.iter().map(|s| seat_id(seat_pos(s))).collect();

        let max = Self::part1(input);
        for i in 1..=max {
            if seats.contains(&(i - 1)) && seats.contains(&(i + 1)) && !seats.contains(&i) {
                return i;
            }
        }
        panic!("I have no seat! Oh no!");
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_day05_part1() {
        let pos = seat_pos("FBFBBFFRLR");
        assert_eq!(pos, (44, 5));
        assert_eq!(seat_id(pos), 357);
    }
}
