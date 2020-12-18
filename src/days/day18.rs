use crate::prelude::*;

pub enum Day18 {}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Add,
    Mul,
    Open,
    Close,
    Num(u64),
}

fn tokenize(s: &str) -> Vec<Token> {
    s.bytes()
        .filter_map(|b| match b {
            b' ' => None,
            b'0'..=b'9' => Some(Token::Num((b - b'0') as _)),
            b'*' => Some(Token::Mul),
            b'+' => Some(Token::Add),
            b'(' => Some(Token::Open),
            b')' => Some(Token::Close),
            _ => panic!("unexpected char: {}", b),
        })
        .collect()
}

// bad shunting yard
fn eval(tokens: impl IntoIterator<Item = Token>, advanced: bool) -> u64 {
    let mut output = Vec::new();
    let mut ops = Vec::new();
    for t in tokens {
        match t {
            n @ Token::Num(_) => output.push(n),
            Token::Add | Token::Mul => {
                while let Some(op) = ops.pop() {
                    if op == Token::Open || (advanced && (t, op) == (Token::Add, Token::Mul)) {
                        ops.push(op);
                        break;
                    } else {
                        output.push(op);
                    }
                }
                ops.push(t);
            }
            Token::Open => ops.push(Token::Open),
            Token::Close => {
                while let Some(op) = ops.pop() {
                    if op == Token::Open {
                        break;
                    } else {
                        output.push(op);
                    }
                }
            }
        }
        //println!("output: {:?}\nops: {:?}", output, ops);
    }
    while let Some(op) = ops.pop() {
        output.push(op);
    }
    //println!("{:?}", output);
    let mut stack = Vec::new();
    for x in output {
        match x {
            Token::Num(n) => stack.push(n),
            Token::Add | Token::Mul => {
                let a = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let c = match x {
                    Token::Add => a + b,
                    Token::Mul => a * b,
                    _ => unreachable!(),
                };
                stack.push(c);
            }
            _ => panic!("wut"),
        }
    }
    stack.into_iter().exactly_one().expect("wat")
}

impl Challenge for Day18 {
    type Input = Vec<Vec<Token>>;
    type Output1 = u64;
    type Output2 = u64;

    fn read(data: File) -> Result<Self::Input, Error> {
        Ok(data.lines().map_results(|s| tokenize(&s)).try_collect()?)
    }

    fn part1(input: Self::Input) -> Self::Output1 {
        input.into_iter().map(|x| eval(x, false)).sum()
    }

    fn part2(input: Self::Input) -> Self::Output2 {
        input.into_iter().map(|x| eval(x, true)).sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn sample_input() -> <Day18 as Challenge>::Input {
        [
            "1 + 2 * 3 + 4 * 5 + 6",
            "1 + (2 * 3) + (4 * (5 + 6))",
            "2 * 3 + (4 * 5)",
            "5 + (8 * 3 + 9 + 3 * 4 * 3)",
            "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
            "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
        ]
        .map(tokenize)
        .to_vec()
    }

    #[test]
    fn test_day18_part1() {
        assert_eq!(
            sample_input()
                .into_iter()
                .map(|x| eval(x, false))
                .collect_vec(),
            [71, 51, 26, 437, 12240, 13632]
        );
    }

    #[test]
    fn test_day18_part2() {
        assert_eq!(
            sample_input()
                .into_iter()
                .map(|x| eval(x, true))
                .collect_vec(),
            [231, 51, 46, 1445, 669060, 23340]
        );
    }
}
