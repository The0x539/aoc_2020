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

pub trait DebugExt: Debug {
    fn dbg(&self) -> String {
        format!("{:?}", self)
    }
}

impl<T: Debug> DebugExt for T {}

pub trait RegexExt {
    fn try_captures<'t>(&self, text: &'t str) -> Result<regex::Captures<'t>, crate::Error>;
}

impl RegexExt for regex::Regex {
    fn try_captures<'t>(&self, text: &'t str) -> Result<regex::Captures<'t>, crate::Error> {
        self.captures(text)
            .ok_or_else(|| crate::Error::RegexFail(self.clone(), text.into()))
    }
}
