use crate::prelude::*;

fn transform_subj_num(subj: u64, loops: u64) -> u64 {
    let mut n = 1;
    for _ in 0..loops {
        n *= subj;
        n %= 20201227;
    }
    n
}

fn derive_loop_size(subj: u64, pubkey: u64) -> u64 {
    let mut n = 1;
    let mut i = 0;
    while n != pubkey {
        n *= subj;
        n %= 20201227;
        i += 1;
    }
    i
}

fn handshake(card_loops: u64, door_loops: u64) -> u64 {
    let card_pubkey = transform_subj_num(7, card_loops);
    let door_pubkey = transform_subj_num(7, door_loops);
    let key_a = transform_subj_num(card_pubkey, door_loops);
    let key_b = transform_subj_num(door_pubkey, card_loops);
    assert_eq!(key_a, key_b);
    key_a
}

pub enum Day25 {}

impl Challenge for Day25 {
    type Input = (u64, u64);
    type Output1 = u64;
    type Output2 = ();

    fn read(data: File) -> Result<Self::Input, Error> {
        Ok(data
            .lines()
            .map(|s| s.unwrap().parse().unwrap())
            .collect_tuple()
            .unwrap())
    }

    fn part1((card_pubkey, door_pubkey): Self::Input) -> Self::Output1 {
        let card_loops = derive_loop_size(7, card_pubkey);
        let door_loops = derive_loop_size(7, door_pubkey);
        handshake(card_loops, door_loops)
    }

    fn part2(_: Self::Input) -> Self::Output2 {}
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day25 as Challenge>::Input {
        (5764801, 17807724)
    }

    #[test]
    fn test_loopsize_derivation() {
        assert_eq!(derive_loop_size(7, 5764801), 8);
        assert_eq!(derive_loop_size(7, 17807724), 11);
    }

    #[test]
    fn test_day25_part1() {
        assert_eq!(Day25::part1(sample_input()), 14897079);
    }
}
