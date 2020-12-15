use std::{fs::File, io::BufReader};

use aoc_2020::{days::*, Challenge};

enum Part {
    Part1,
    Part2,
}

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
            let input = <$day>::read(data).unwrap();
            match part {
                Part::Part1 => format!("{:?}", <$day>::part1(input)),
                Part::Part2 => format!("{:?}", <$day>::part2(input)),
            }
        }};
    }

    let output = match day {
        1 => day!(day01::Day01),
        2 => day!(day02::Day02),
        3 => day!(day03::Day03),
        4 => day!(day04::Day04),
        5 => day!(day05::Day05),
        6 => day!(day06::Day06),
        7 => day!(day07::Day07),
        8 => day!(day08::Day08),
        9 => day!(day09::Day09),
        10 => day!(day10::Day10),
        11 => day!(day11::Day11),
        12 => day!(day12::Day12),
        13 => day!(day13::Day13),
        14 => day!(day14::Day14),
        15 => day!(day15::Day15),
        _ => panic!("no such day"),
    };
    println!("{}", output);
}
