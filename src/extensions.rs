pub use itertools::Itertools;

pub trait StrExt {
    fn nth_char(&self, n: usize) -> char;
}

impl StrExt for str {
    fn nth_char(&self, n: usize) -> char {
        self.chars().nth(n).unwrap()
    }
}
