use crate::prelude::*;

use std::collections::{HashSet, VecDeque};

pub enum Day22 {}

#[derive(PartialEq)]
enum Winner {
    P1,
    P2,
}

fn play_game(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> Winner {
    while !(p1.is_empty() || p2.is_empty()) {
        let p1_card = p1.pop_front().unwrap();
        let p2_card = p2.pop_front().unwrap();
        if p1_card > p2_card {
            p1.push_back(p1_card);
            p1.push_back(p2_card);
        } else if p2_card > p1_card {
            p2.push_back(p2_card);
            p2.push_back(p1_card);
        } else {
            panic!("wat");
        }
    }
    if p1.is_empty() {
        Winner::P2
    } else {
        Winner::P1
    }
}

fn play_rec_game(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> Winner {
    let mut states = HashSet::<(Vec<usize>, Vec<usize>)>::new();

    while !(p1.is_empty() || p2.is_empty()) {
        let state = (p1.iter().copied().collect(), p2.iter().copied().collect());
        if !states.insert(state) {
            return Winner::P1;
        }

        let p1_card = p1.pop_front().unwrap();
        let p2_card = p2.pop_front().unwrap();

        let round_winner = if p1_card <= p1.len() && p2_card <= p2.len() {
            p1.make_contiguous();
            p2.make_contiguous();
            let mut p1_subdeck = p1.as_slices().0[..p1_card].to_vec().into();
            let mut p2_subdeck = p2.as_slices().0[..p2_card].to_vec().into();
            play_rec_game(&mut p1_subdeck, &mut p2_subdeck)
        } else {
            if p1_card > p2_card {
                Winner::P1
            } else if p2_card > p1_card {
                Winner::P2
            } else {
                panic!("wat");
            }
        };

        match round_winner {
            Winner::P1 => {
                p1.push_back(p1_card);
                p1.push_back(p2_card);
            }
            Winner::P2 => {
                p2.push_back(p2_card);
                p2.push_back(p1_card);
            }
        }
    }
    if p1.is_empty() {
        Winner::P2
    } else {
        Winner::P1
    }
}

impl Challenge for Day22 {
    type Input = (VecDeque<usize>, VecDeque<usize>);
    type Output1 = usize;
    type Output2 = usize;

    fn read(data: File) -> Result<Self::Input, Error> {
        let mut lines = data.lines();
        lines.next();
        let mut p1 = VecDeque::new();
        for line in &mut lines {
            let line = line?;
            if line == "" {
                break;
            }
            p1.push_back(line.parse()?);
        }
        lines.next();
        let mut p2 = VecDeque::new();
        for line in lines {
            p2.push_back(line?.parse()?);
        }
        Ok((p1, p2))
    }

    fn part1((mut p1, mut p2): Self::Input) -> Self::Output1 {
        let winner = match play_game(&mut p1, &mut p2) {
            Winner::P1 => p1,
            Winner::P2 => p2,
        };

        winner
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, v)| (i + 1) * v)
            .sum()
    }

    fn part2((mut p1, mut p2): Self::Input) -> Self::Output2 {
        let winner = match play_rec_game(&mut p1, &mut p2) {
            Winner::P1 => p1,
            Winner::P2 => p2,
        };

        winner
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, v)| (i + 1) * v)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day22 as Challenge>::Input {
        (vec![9, 2, 6, 3, 1].into(), vec![5, 8, 4, 7, 10].into())
    }

    #[test]
    fn test_day22_part1() {
        assert_eq!(Day22::part1(sample_input()), 306);
    }

    #[test]
    fn test_day22_part2() {
        assert_eq!(Day22::part2(sample_input()), 291);
    }
}
