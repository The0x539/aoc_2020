use std::{fmt::Debug, io::BufRead, iter::FromIterator, str::FromStr};

pub use itertools::Itertools;

pub trait StrExt {
    fn nth_char(&self, n: usize) -> char;
}

impl StrExt for str {
    fn nth_char(&self, n: usize) -> char {
        self.chars().nth(n).unwrap()
    }
}

pub trait BufReadExt: BufRead + Sized {
    fn parse_lines<T, V>(self) -> V
    where
        T: FromStr,
        T::Err: Debug,
        V: FromIterator<T>,
    {
        self.lines().map(|s| s.unwrap().parse().unwrap()).collect()
    }
}

impl<T: BufRead> BufReadExt for T {}
