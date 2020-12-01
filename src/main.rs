use std::{fs::File, io::BufReader};

use aoc_2020::{days::*, Challenge};

enum Part {
    Part1,
    Part2,
}

trait DebugExt: std::fmt::Debug {
    fn dbg(&self) -> String {
        format!("{:?}", self)
    }
}

impl<T: std::fmt::Debug> DebugExt for T {}

fn main() {
    let mut args = std::env::args();
    let _ = args.next().unwrap();
    let day = args.next().unwrap().parse::<u8>().unwrap();
    let part = match args.next().unwrap().as_str() {
        "1" => Part::Part1,
        "2" => Part::Part2,
        _ => panic!(),
    };

    let data = BufReader::new(File::open(format!("input/day{:02}.txt", day)).unwrap());

    macro_rules! day {
        ($day:ty) => {{
            let input = <$day>::read(data);
            match part {
                Part::Part1 => format!("{:?}", <$day>::part1(input)),
                Part::Part2 => format!("{:?}", <$day>::part2(input)),
            }
        }};
    }

    let output = match day {
        1 => day!(day01::Day01),
        _ => panic!("no such day"),
    };
    println!("{}", output);
}
