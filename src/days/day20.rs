use crate::prelude::*;

use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use itertools::{iproduct, izip};

#[derive(Copy, Clone)]
pub struct Tile<const N: usize> {
    pixels: [[bool; N]; N],
}

impl<const N: usize> Tile<N> {
    fn rot_r(&self) -> Self {
        let mut pixels = [[false; N]; N];

        for (x, y) in iproduct!(0..N, 0..N) {
            pixels[y][x] = self.pixels[(N - 1) - x][y];
        }

        Self { pixels }
    }

    fn flip_v(&self) -> Self {
        let mut pixels = [[false; N]; N];

        for y in 0..N {
            pixels[y] = self.pixels[(N - 1) - y];
        }

        Self { pixels }
    }

    fn orientations(mut self) -> Vec<Self> {
        let mut os = Vec::new();
        for _ in 0..4 {
            os.push(self);
            self = self.rot_r();
        }
        self = self.flip_v();
        for _ in 0..4 {
            os.push(self);
            self = self.rot_r();
        }
        os
    }

    fn fits_above(&self, other: &Self) -> bool {
        self.pixels[N - 1] == other.pixels[0]
    }

    fn fits_below(&self, other: &Self) -> bool {
        other.fits_above(self)
    }

    fn fits_left(&self, other: &Self) -> bool {
        (0..N).all(|y| self.pixels[y][N - 1] == other.pixels[y][0])
    }

    fn fits_right(&self, other: &Self) -> bool {
        other.fits_left(self)
    }
}

impl<const N: usize> std::fmt::Display for Tile<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buf = vec![b'.'; (N + 1) * N - 1];
        for (x, y) in iproduct!(0..N, 0..N) {
            if self.pixels[y][x] {
                buf[(N + 1) * y + x] = b'#';
            }
        }
        for y in 0..(N - 1) {
            buf[(N + 1) * y + N] = b'\n';
        }
        f.write_str(std::str::from_utf8(&buf).unwrap())
    }
}

impl<const N: usize> TryFrom<Vec<Vec<bool>>> for Tile<N> {
    type Error = &'static str; // whatever

    fn try_from(value: Vec<Vec<bool>>) -> Result<Self, Self::Error> {
        let pixels = value
            .into_iter()
            .map(|row| row.try_into().map_err(|_| "bad width"))
            .collect::<Result<Vec<[bool; N]>, &str>>()?
            .try_into()
            .map_err(|_| "bad height")?;

        Ok(Self { pixels })
    }
}

fn parse(s: &str) -> Result<<Day20 as Challenge>::Input, Error> {
    let mut tiles = HashMap::new();
    for mut chunk in &s.split('\n').chunks(12) {
        let id_line = chunk.next().ok_or("no id???")?;
        if id_line.is_empty() {
            break;
        }
        let id = id_line[5..(id_line.len() - 1)].parse()?;
        let tile = chunk
            .take(10)
            .map(|s| s.bytes().map(|c| c == b'#').collect_vec())
            .collect_vec()
            .try_into()?;

        tiles.insert(id, tile);
    }
    Ok(tiles)
}

fn assemble(
    mut tiles: HashMap<usize, Tile<10>>,
) -> (HashMap<(i8, i8), Tile<10>>, HashMap<(i8, i8), usize>) {
    let mut img = HashMap::new();
    let mut ids = HashMap::new();

    let first_id = tiles.keys().copied().next().unwrap();
    ids.insert((0, 0), first_id);

    let first_tile = tiles.remove(&first_id).unwrap();
    img.insert((0, 0), first_tile);

    while tiles.len() > 0 {
        let mut failed = HashMap::new();

        'foo: for (id, tile) in tiles {
            for piece in tile.orientations() {
                for (x, y) in img.keys().copied().collect_vec() {
                    let pos = (x, y);
                    let (above, below, left, right) =
                        ((x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y));

                    if !img.contains_key(&above) && piece.fits_above(&img[&pos]) {
                        ids.insert(above, id);
                        img.insert(above, piece);
                    } else if !img.contains_key(&below) && piece.fits_below(&img[&pos]) {
                        ids.insert(below, id);
                        img.insert(below, piece);
                    } else if !img.contains_key(&left) && piece.fits_left(&img[&pos]) {
                        ids.insert(left, id);
                        img.insert(left, piece);
                    } else if !img.contains_key(&right) && piece.fits_right(&img[&pos]) {
                        ids.insert(right, id);
                        img.insert(right, piece);
                    } else {
                        continue;
                    }
                    continue 'foo;
                }
            }

            failed.insert(id, tile);
        }

        tiles = failed;
    }

    (img, ids)
}

fn row_match(pixels: &[bool], monster: &[u8]) -> bool {
    pixels
        .iter()
        .zip(monster.iter())
        .all(|(v, c)| *c == b' ' || *v)
}

fn do_part2<const N: usize>(img: Tile<N>) -> usize {
    let num_hash = img
        .pixels
        .iter()
        .flat_map(|row| row.iter())
        .filter(|x| **x)
        .count();

    let monster = [
        b"                  # ",
        b"#    ##    ##    ###",
        b" #  #  #  #  #  #   ",
    ];

    for img in img.orientations() {
        let mut monster_count = 0;
        for rows in img.pixels.windows(monster.len()) {
            let (t, m, b) = match rows {
                [t, m, b] => (t, m, b),
                _ => unreachable!(),
            };

            let w = monster[0].len();
            for (t, m, b) in izip!(t.windows(w), m.windows(w), b.windows(w)) {
                if izip!(&[t, m, b], &monster).all(|(p, m)| row_match(p, *m)) {
                    monster_count += 1;
                }
            }
        }

        if monster_count != 0 {
            return num_hash - 15 * monster_count;
        }
    }

    println!("no monsters found");
    num_hash
}

pub enum Day20 {}

impl Challenge for Day20 {
    type Input = HashMap<usize, Tile<10>>;
    type Output1 = usize;
    type Output2 = usize;

    fn read(mut data: File) -> Result<Self::Input, Error> {
        let mut s = String::new();
        data.read_to_string(&mut s)?;
        parse(&s)
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        let (_, ids) = assemble(input);

        let (min_x, max_x) = ids.keys().map(|(x, _)| *x).minmax().into_option().unwrap();
        let (min_y, max_y) = ids.keys().map(|(_, y)| *y).minmax().into_option().unwrap();

        ids[&(min_x, min_y)] * ids[&(min_x, max_y)] * ids[&(max_x, min_y)] * ids[&(max_x, max_y)]
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        let (img, _) = assemble(input);

        let (min_x, max_x) = img.keys().map(|(x, _)| *x).minmax().into_option().unwrap();
        let (min_y, max_y) = img.keys().map(|(_, y)| *y).minmax().into_option().unwrap();

        let mut stitched = Vec::new();

        for tile_y in min_y..=max_y {
            let mut rows = vec![Vec::new(); 8];
            for tile_x in min_x..=max_x {
                let tile = &img[&(tile_x, tile_y)];
                for y in 1..9 {
                    rows[y - 1].extend(tile.pixels[y][1..9].iter().copied());
                }
            }
            stitched.extend(rows);
        }

        match stitched.len() {
            24 => do_part2::<24>(stitched.try_into().unwrap()),
            96 => do_part2::<96>(stitched.try_into().unwrap()),
            _ => panic!("unsupported dimensions"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day20 as Challenge>::Input {
        let s = std::fs::read_to_string("input/day20_sample.txt").unwrap();
        parse(&s).unwrap()
    }

    #[test]
    fn test_day20_part1() {
        assert_eq!(Day20::part1(sample_input()), 20899048083289);
    }

    #[test]
    fn test_day20_part2() {
        assert_eq!(Day20::part2(sample_input()), 273);
    }
}
