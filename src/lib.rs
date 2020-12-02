use std::{fmt::Debug, fs::File, io::BufReader};

pub trait Challenge {
    type Input;
    type Output1: Debug;
    type Output2: Debug;

    fn read(data: BufReader<File>) -> Self::Input;

    fn part1(input: Self::Input) -> Self::Output1;
    fn part2(input: Self::Input) -> Self::Output2;
}

pub mod days;
pub mod prelude;
