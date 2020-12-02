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
    fn parse_lines<T, V>(self) -> Result<V, crate::Error>
    where
        T: FromStr,
        T::Err: Debug,
        crate::Error: From<T::Err>,
        V: FromIterator<T>,
    {
        self.lines().map(|s| Ok(s?.parse()?)).collect()
    }
}

impl<T: BufRead> BufReadExt for T {}
