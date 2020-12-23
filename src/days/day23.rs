use crate::prelude::*;

use std::collections::VecDeque;

pub enum Day23 {}

fn do_move(cups: &mut VecDeque<u32>) {
    //println!("cups: {:?}", cups);
    let (min, max) = (1, cups.len() as u32);
    cups.rotate_left(1);
    let picked_cups = [
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
        cups.pop_front().unwrap(),
    ];
    cups.rotate_right(1);
    //println!("pick up: {:?}", picked_cups);
    let mut dest_cup = cups[0] - 1;
    if dest_cup < min {
        dest_cup = max;
    }
    while picked_cups.iter().any(|v| *v == dest_cup) {
        dest_cup -= 1;
        if dest_cup < min {
            dest_cup = max;
        }
    }
    //println!("destination: {}", dest_cup);

    /*
    let i = cups
        .iter()
        .copied()
        .enumerate()
        .find(|(_, v)| *v == dest_cup)
        .unwrap()
        .0;
    */
    let left = 0..cups.len();
    let right = left.clone().rev();
    let i = left
        .interleave(right)
        .find(|i| cups[*i] == dest_cup)
        .unwrap();

    //println!("{:?}, i={}", cups, i);

    /*
    cups.rotate_left(i + 1);
    cups.push_back(picked_cups[0]);
    cups.push_back(picked_cups[1]);
    cups.push_back(picked_cups[2]);
    cups.rotate_right(i + 3);
    */
    cups.rotate_left(1);
    cups.insert(i, picked_cups[2]);
    cups.insert(i, picked_cups[1]);
    cups.insert(i, picked_cups[0]);
}

impl Challenge for Day23 {
    type Input = VecDeque<u32>;
    type Output1 = VecDeque<u32>;
    type Output2 = u64;

    fn read(data: File) -> Result<Self::Input, Error> {
        let v = data
            .bytes()
            .map(Result::unwrap)
            .filter(u8::is_ascii_digit)
            .map(|n| (n - b'0') as u32)
            .collect_vec()
            .into();

        Ok(v)
    }

    fn part1(mut input: Self::Input) -> Self::Output1 {
        for _ in 0..100 {
            do_move(&mut input);
        }
        while input[0] != 1 {
            input.rotate_right(1);
        }
        input.pop_front();
        input
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        let mut cups = input;
        for i in 10..=1_000_000 {
            cups.push_back(i);
        }
        for i in 0..10_000_000 {
            if i % 10000 == 0 {
                println!("{}", i);
            }
            do_move(&mut cups);
        }
        while cups[0] != 1 {
            cups.rotate_right(1);
        }
        cups[1] as u64 * cups[2] as u64
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day23 as Challenge>::Input {
        vec![3, 8, 9, 1, 2, 5, 4, 6, 7].into()
    }

    #[test]
    fn test_day23_part1() {
        assert_eq!(Day23::part1(sample_input()), vec![6, 7, 3, 8, 4, 5, 2, 9]);
    }

    #[test]
    fn test_day23_part2() {
        assert_eq!(Day23::part2(sample_input()), 149245887792);
    }
}
