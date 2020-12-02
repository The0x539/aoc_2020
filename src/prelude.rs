pub use std::{
    io::{BufRead, Read},
    str::FromStr,
};

pub use crate::Challenge;

pub type File = std::io::BufReader<std::fs::File>;
