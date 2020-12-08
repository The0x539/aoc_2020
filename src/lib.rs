use std::{fmt::Debug, fs::File, io::BufReader};
use thiserror::Error;

pub trait Challenge {
    type Input;
    type Output1: Debug;
    type Output2: Debug;

    fn read(data: BufReader<File>) -> Result<Self::Input, Error>;

    fn part1(input: Self::Input) -> Self::Output1;
    fn part2(input: Self::Input) -> Self::Output2;
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("integer parse error")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("Regex \"{0}\" didn't match \"{1}\"")]
    RegexFail(regex::Regex, String),
    #[error("{0}")]
    Generic(&'static str),
}

pub mod days;
pub mod extensions;
pub mod prelude;
