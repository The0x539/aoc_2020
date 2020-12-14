use crate::prelude::*;

use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum Instruction {
    Mask([Option<bool>; 36]),
    Write(u64, u64),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[..3] {
            "mas" => {
                let mut mask = [None; 36];
                for (i, c) in s[7..].chars().enumerate() {
                    mask[i] = match c {
                        '0' => Some(false),
                        '1' => Some(true),
                        'X' => None,
                        _ => Err("bad mask bit")?,
                    };
                }
                Ok(Self::Mask(mask))
            }
            "mem" => {
                let bracket = s.find(']').ok_or("bad write syntax")?;
                let addr = s[4..bracket].parse()?;
                let val = s[bracket + 4..].parse()?;
                Ok(Self::Write(addr, val))
            }
            _ => Err("bad instr")?,
        }
    }
}

fn apply_mask(mut val: u64, mask: &[Option<bool>; 36]) -> u64 {
    for i in 0..36 {
        if let Some(bit) = mask[35 - i] {
            let b = 1 << i;
            if bit {
                val |= b;
            } else {
                val &= !b;
            }
        }
    }
    val
}

fn what(mut addr: u64, mask: &[Option<bool>; 36]) -> Vec<u64> {
    let mut floating_bits = Vec::new();
    for i in 0..36 {
        let b = 1 << i;
        match mask[35 - i] {
            Some(true) => addr |= b,
            Some(false) => (),
            None => floating_bits.push(b),
        }
    }
    let mut addrs = vec![addr];
    for b in floating_bits {
        for a in &mut addrs {
            *a |= b;
        }
        let mut zeroes = addrs.clone();
        for a in &mut zeroes {
            *a &= !b;
        }
        addrs.extend(zeroes);
    }
    addrs
}

pub enum Day14 {}

impl Challenge for Day14 {
    type Input = Vec<Instruction>;
    type Output1 = u64;
    type Output2 = u64;

    fn read(data: File) -> Result<Self::Input, Error> {
        data.parse_lines()
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        let mut memory = HashMap::new();
        let mut mask = [None; 36];
        for instr in input {
            match instr {
                Instruction::Mask(m) => mask = m,
                Instruction::Write(addr, val) => {
                    let x = memory.entry(addr).or_default();
                    *x = apply_mask(val, &mask);
                }
            }
        }
        memory.values().sum()
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        let mut memory = HashMap::new();
        let mut mask = [None; 36];
        for instr in input {
            match instr {
                Instruction::Mask(m) => mask = m,
                Instruction::Write(addr, val) => {
                    for a in what(addr, &mask) {
                        let x = memory.entry(a).or_default();
                        *x = val;
                    }
                }
            }
        }
        memory.values().sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input1() -> <Day14 as Challenge>::Input {
        [
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ]
        .iter()
        .copied()
        .map(|s| s.parse().unwrap())
        .collect()
    }

    fn sample_input2() -> <Day14 as Challenge>::Input {
        [
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ]
        .iter()
        .copied()
        .map(|s| s.parse().unwrap())
        .collect()
    }

    #[test]
    fn test_day14_part1() {
        assert_eq!(Day14::part1(sample_input1()), 165);
    }

    #[test]
    fn test_day14_part2() {
        assert_eq!(Day14::part2(sample_input2()), 208);
    }
}
