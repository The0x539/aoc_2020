use crate::prelude::*;

use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

fn take_byte(b: &mut &[u8]) -> u8 {
    let v = b[0];
    *b = &b[1..];
    v
}

impl Direction {
    fn from_bytes(mut b: &[u8]) -> Vec<Self> {
        let mut v = Vec::new();
        while !b.is_empty() {
            let dir = match take_byte(&mut b) {
                b'e' => Self::East,
                b'w' => Self::West,
                c => match (c, take_byte(&mut b)) {
                    (b's', b'e') => Self::Southeast,
                    (b's', b'w') => Self::Southwest,
                    (b'n', b'w') => Self::Northwest,
                    (b'n', b'e') => Self::Northeast,
                    _ => panic!("wat"),
                },
            };
            v.push(dir);
        }
        v
    }

    fn to_offset(self) -> (i32, i32) {
        match self {
            Self::East => (2, 0),
            Self::Southeast => (1, 2),
            Self::Southwest => (-1, 2),
            Self::West => (-2, 0),
            Self::Northwest => (-1, -2),
            Self::Northeast => (1, -2),
        }
    }
}

fn flipped_tiles(input: <Day24 as Challenge>::Input) -> HashMap<(i32, i32), bool> {
    let mut tiles = HashMap::<_, bool>::new();
    for identifier in input {
        let (mut x, mut y) = (0, 0);
        for dir in identifier {
            let (dx, dy) = dir.to_offset();
            x += dx;
            y += dy;
        }
        let entry = tiles.entry((x, y)).or_default();
        *entry = !*entry;
    }
    tiles
}

fn next_gen(prev: &HashMap<(i32, i32), bool>) -> HashMap<(i32, i32), bool> {
    let mut neighbor_counts = prev
        .keys()
        .copied()
        .map(|k| (k, 0))
        .collect::<HashMap<_, _>>();

    for (&pos, &black) in prev.iter() {
        if black {
            for dir in Direction::from_bytes(b"eseswwnwne") {
                let (dx, dy) = dir.to_offset();
                let pos2 = (pos.0 + dx, pos.1 + dy);
                let n = neighbor_counts.entry(pos2).or_default();
                *n += 1;
            }
        }
    }

    neighbor_counts
        .into_iter()
        .map(|(pos, n)| {
            let is_black = n == 2 || (n == 1 && prev.get(&pos) == Some(&true));
            (pos, is_black)
        })
        .collect()
}

pub enum Day24 {}

impl Challenge for Day24 {
    type Input = Vec<Vec<Direction>>;
    type Output1 = usize;
    type Output2 = usize;

    fn read(data: File) -> Result<Self::Input, Error> {
        let mut v = Vec::new();
        for line in data.lines() {
            v.push(Direction::from_bytes(line?.as_bytes()));
        }
        Ok(v)
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        flipped_tiles(input).values().filter(|x| **x).count()
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        let mut tiles = flipped_tiles(input);
        for _ in 0..100 {
            tiles = next_gen(&tiles);
        }
        tiles.values().filter(|x| **x).count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day24 as Challenge>::Input {
        [
            "sesenwnenenewseeswwswswwnenewsewsw",
            "neeenesenwnwwswnenewnwwsewnenwseswesw",
            "seswneswswsenwwnwse",
            "nwnwneseeswswnenewneswwnewseswneseene",
            "swweswneswnenwsewnwneneseenw",
            "eesenwseswswnenwswnwnwsewwnwsene",
            "sewnenenenesenwsewnenwwwse",
            "wenwwweseeeweswwwnwwe",
            "wsweesenenewnwwnwsenewsenwwsesesenwne",
            "neeswseenwwswnwswswnw",
            "nenwswwsewswnenenewsenwsenwnesesenew",
            "enewnwewneswsewnwswenweswnenwsenwsw",
            "sweneswneswneneenwnewenewwneswswnese",
            "swwesenesewenwneswnwwneseswwne",
            "enesenwswwswneneswsenwnewswseenwsese",
            "wnwnesenesenenwwnenwsewesewsesesew",
            "nenewswnwewswnenesenwnesewesw",
            "eneswnwswnwsenenwnwnwwseeswneewsenese",
            "neswnwewnwnwseenwseesewsenwsweewe",
            "wseweeenwnesenwwwswnew",
        ]
        .map(str::as_bytes)
        .map(Direction::from_bytes)
        .to_vec()
    }

    #[test]
    fn test_day24_part1() {
        assert_eq!(Day24::part1(sample_input()), 10);
    }

    #[test]
    fn test_day24_part2() {
        assert_eq!(Day24::part2(sample_input()), 2208);
    }
}
