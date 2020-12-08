use crate::prelude::*;

enum Exit {
    Loop(i32),
    Terminate(i32),
    Segfault,
}

#[derive(Copy, Clone, PartialEq)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Copy, Clone)]
pub struct Instruction {
    op: Operation,
    arg: i32,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let op = match &s[..3] {
            "nop" => Operation::Nop,
            "acc" => Operation::Acc,
            "jmp" => Operation::Jmp,
            _ => return Err(Error::Generic("bad opcode")),
        };
        let arg = s[4..].parse()?;
        Ok(Self { op, arg })
    }
}

pub enum Day08 {}

fn run(prog: &[Instruction]) -> Exit {
    let mut acc = 0;

    let mut executed_instrs = vec![0; prog.len()];
    let mut ip: i32 = 0;

    loop {
        if ip == prog.len() as i32 {
            break Exit::Terminate(acc);
        } else if ip > prog.len() as i32 {
            break Exit::Segfault;
        } else if executed_instrs[ip as usize] != 0 {
            break Exit::Loop(acc);
        }

        executed_instrs[ip as usize] += 1;

        let instr = prog[ip as usize];

        if instr.op == Operation::Acc {
            acc += instr.arg;
        }

        ip += match instr.op {
            Operation::Jmp => instr.arg,
            _ => 1,
        };
    }
}

impl Challenge for Day08 {
    type Input = Vec<Instruction>;
    type Output1 = i32;
    type Output2 = i32;

    fn read(data: File) -> Result<Self::Input, Error> {
        data.parse_lines()
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        match run(&input) {
            Exit::Loop(acc) => acc,
            _ => panic!("Program should have looped"),
        }
    }

    fn part2(mut input: Self::Input) -> Self::Output2 {
        for i in 0..input.len() {
            let mut swapped_op = match input[i].op {
                Operation::Nop => Operation::Jmp,
                Operation::Jmp => Operation::Nop,
                Operation::Acc => continue,
            };

            std::mem::swap(&mut swapped_op, &mut input[i].op);

            if let Exit::Terminate(acc) = run(&input) {
                return acc;
            }

            // restore the original op for future iterations
            input[i].op = swapped_op;
        }

        panic!("No modified versions terminated")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day08 as Challenge>::Input {
        [
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .iter()
        .copied()
        .map(|s| s.parse().unwrap())
        .collect()
    }

    #[test]
    fn test_day08_part1() {
        assert_eq!(Day08::part1(sample_input()), 5);
    }

    #[test]
    fn test_day08_part2() {
        assert_eq!(Day08::part2(sample_input()), 8);
    }
}
