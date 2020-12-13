use crate::prelude::*;

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn left(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }

    fn right(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    fn vec(self) -> (i64, i64) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }
}

#[derive(Copy, Clone)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Copy, Clone)]
pub struct Instruction {
    action: Action,
    amount: i64,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s.nth_char(0) {
            'N' => Action::North,
            'S' => Action::South,
            'E' => Action::East,
            'W' => Action::West,
            'L' => Action::Left,
            'R' => Action::Right,
            'F' => Action::Forward,
            _ => return Err(Error::Generic("bad action")),
        };

        let mut amount = s[1..].parse()?;
        if matches!(action, Action::Left | Action::Right) {
            amount /= 90;
        }
        Ok(Self { action, amount })
    }
}

pub enum Day12 {}

impl Challenge for Day12 {
    type Input = Vec<Instruction>;
    type Output1 = i64;
    type Output2 = i64;

    fn read(data: File) -> Result<Self::Input, Error> {
        data.parse_lines()
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        let mut bearing = Direction::East;
        let mut x = 0;
        let mut y = 0;
        for instruction in input {
            let amt = instruction.amount;
            match instruction.action {
                Action::North => y -= amt,
                Action::South => y += amt,
                Action::East => x += amt,
                Action::West => x -= amt,
                Action::Left => (0..amt).for_each(|_| bearing = bearing.left()),
                Action::Right => (0..amt).for_each(|_| bearing = bearing.right()),
                Action::Forward => {
                    let (dx, dy) = bearing.vec();
                    x += dx * amt;
                    y += dy * amt;
                }
            }
        }

        x.abs() + y.abs()
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        let mut waypoint = (10, -1);
        let mut x = 0;
        let mut y = 0;
        for instruction in input {
            let amt = instruction.amount;
            match instruction.action {
                Action::North => waypoint.1 -= amt,
                Action::South => waypoint.1 += amt,
                Action::East => waypoint.0 += amt,
                Action::West => waypoint.0 -= amt,
                Action::Left => {
                    for _ in 0..amt {
                        waypoint = (waypoint.1, -waypoint.0);
                    }
                }
                Action::Right => {
                    for _ in 0..amt {
                        waypoint = (-waypoint.1, waypoint.0);
                    }
                }
                Action::Forward => {
                    for _ in 0..amt {
                        x += waypoint.0;
                        y += waypoint.1;
                    }
                }
            }
        }

        x.abs() + y.abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day12 as Challenge>::Input {
        ["F10", "N3", "F7", "R90", "F11"]
            .iter()
            .copied()
            .map(|s| s.parse().unwrap())
            .collect()
    }

    #[test]
    fn test_day12_part1() {
        assert_eq!(Day12::part1(sample_input()), 25);
    }

    #[test]
    fn test_day12_part2() {
        assert_eq!(Day12::part2(sample_input()), 286);
    }
}
