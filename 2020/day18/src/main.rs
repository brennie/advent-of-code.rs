use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;

fn main() -> Result<()> {
    let input: Vec<String> = BufReader::new(File::open("input")?)
        .lines()
        .map(|r| r.map_err(Into::into))
        .collect::<Result<_>>()?;

    {
        let sum: usize = input.iter().map(|s| eval(&s, equal_precedence)).sum();

        println!("part 1: {}", sum)
    }
    {
        let sum: usize = input.iter().map(|s| eval(&s, weird_precedence)).sum();

        println!("part 1: {}", sum)
    }

    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Op {
    Add,
    Mul,
    Open,
    Close,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Token {
    Operator(Op),
    Number(usize),
}

#[derive(Debug)]
enum Output {
    Operator(Op),
    Number(usize),
}

fn equal_precedence(o: Op) -> usize {
    match o {
        Op::Add => 1,
        Op::Mul => 1,
        _ => unreachable!(),
    }
}

fn weird_precedence(o: Op) -> usize {
    match o {
        Op::Add => 2,
        Op::Mul => 1,
        _ => unreachable!(),
    }
}

fn eval<F>(s: &str, precedence: F) -> usize
where
    F: Fn(Op) -> usize,
{
    let s = s.trim();

    let mut tokens = s.char_indices().peekable();
    let mut output = VecDeque::new();
    let mut operators = VecDeque::new();

    while let Some((i, c)) = tokens.next() {
        let token = match c {
            '+' => Token::Operator(Op::Add),
            '*' => Token::Operator(Op::Mul),
            '(' => Token::Operator(Op::Open),
            ')' => Token::Operator(Op::Close),
            ' ' => {
                continue;
            }

            c if c.is_ascii_digit() => {
                let start = i;
                let mut end = start;

                while let Some((j, d)) = tokens.peek().cloned() {
                    if d.is_ascii_digit() {
                        tokens.next();
                        end = j;
                    } else {
                        break;
                    }
                }

                let n = s[start..=end].parse().unwrap();
                Token::Number(n)
            }

            _ => panic!("unexpected token {}", c),
        };

        match token {
            Token::Number(..) => output.push_back(token),
            Token::Operator(o) => match o {
                Op::Open => operators.push_back(o),
                Op::Close => {
                    while let Some(top) = operators.back().cloned() {
                        if top == Op::Open {
                            break;
                        }

                        output.push_back(Token::Operator(top));
                        operators.pop_back();
                    }

                    if operators.len() == 0 {
                        panic!("mismatched parentheses");
                    }

                    assert_eq!(operators.pop_back().unwrap(), Op::Open);
                }
                _ => {
                    while let Some(top) = operators.back().cloned() {
                        if top == Op::Open {
                            break;
                        }

                        if precedence(top) > precedence(o) {
                            output.push_back(Token::Operator(top));
                            operators.pop_back();
                        } else {
                            break;
                        }
                    }
                    operators.push_back(o);
                }
            },
        }
    }

    while let Some(o) = operators.pop_back() {
        output.push_back(Token::Operator(o));
    }

    let mut values = VecDeque::new();
    while let Some(token) = output.pop_front() {
        match token {
            Token::Number(n) => values.push_back(n),
            Token::Operator(o) => {
                let v = match o {
                    Op::Open | Op::Close => unreachable!(),
                    Op::Add => values.pop_back().unwrap() + values.pop_back().unwrap(),
                    Op::Mul => values.pop_back().unwrap() * values.pop_back().unwrap(),
                };
                values.push_back(v);
            }
        }
    }

    assert_eq!(values.len(), 1);
    return values[0];
}
