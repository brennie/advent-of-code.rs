use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use combine::parser::char::{char, digit, space};
use combine::parser::choice::{choice, optional};
use combine::parser::combinator::{attempt, no_partial};
use combine::parser::range::{range, recognize};
use combine::parser::repeat::skip_many1;
use combine::parser::token::{any, value};
use combine::{ParseError, Parser, RangeStream};

type Rules = HashMap<usize, Production>;

fn main() -> Result<()> {
    let (rules, messages) = parse_input()?;

    println!("part 1: {}", count_matches(&rules, &messages));

    {
        let rules = {
            let mut rules = rules.clone();
            rules.insert(8, Production::Plus(42));

            let max_len = messages.iter().map(String::len).max().unwrap();
            let mut balanced = vec![];
            for i in 1..=max_len / 2 {
                let mut rule = vec![];
                for _ in 0..i {
                    rule.push(42);
                }

                for _ in 0..i {
                    rule.push(31);
                }
                balanced.push(Production::SeqN(rule));
            }

            rules.insert(11, Production::OrN(balanced));
            rules
        };

        println!("part 2: {}", count_matches(&rules, &messages));
    }

    Ok(())
}

fn count_matches(rules: &Rules, messages: &[String]) -> usize {
    let mut parser = parser_for_rule(&rules[&0], &rules);
    messages
        .iter()
        .filter(|msg| match parser.parse(msg.as_str()) {
            Ok((_, "")) => true,
            _ => false,
        })
        .count()
}

fn parse_input() -> anyhow::Result<(Rules, Vec<String>)> {
    enum State {
        Rules,
        Messages,
    }

    let mut state = State::Rules;
    let mut rules = HashMap::new();
    let mut messages: Vec<String> = vec![];
    for line in BufReader::new(File::open("input")?).lines() {
        let line = line?;

        match state {
            State::Rules => {
                if line.len() == 0 {
                    state = State::Messages;
                    continue;
                }

                let ((id, rule), rest) = rule().parse(&*line).unwrap();
                assert!(rest.is_empty());

                rules.insert(id, rule);
            }

            State::Messages => {
                messages.push(line);
            }
        }
    }

    Ok((rules, messages))
}

#[derive(Clone)]
enum Production {
    Char(char),
    Ref(usize),
    Seq(usize, usize),
    SeqN(Vec<usize>),
    Plus(usize),
    Or(Box<Production>, Box<Production>),
    OrN(Vec<Production>),
}

fn rule<'a, I>() -> impl Parser<I, Output = (usize, Production)> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    let number = || recognize(skip_many1(digit())).map(|s: &str| s.parse::<usize>().unwrap());
    let string = || char('"').with(any()).skip(char('"')).map(Production::Char);

    let ref_seq = || {
        (number(), optional(attempt(space().with(number())))).map(|(a, b)| match (a, b) {
            (a, Some(b)) => Production::Seq(a, b),
            (a, None) => Production::Ref(a),
        })
    };

    let ref_seq_or =
        (ref_seq(), optional(range(" | ").with(ref_seq()))).map(|(a, b)| match (a, b) {
            (a, Some(b)) => Production::Or(Box::new(a), Box::new(b)),
            (a, None) => a,
        });

    let rule = || Parser::or(string(), ref_seq_or);

    (number().skip(range(": ")), rule())
}

fn parser_for_rule<'a, I>(
    rule: &Production,
    rules: &Rules,
) -> Box<dyn Parser<I, Output = (), PartialState = ()> + 'a>
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    match rule {
        Production::Char(c) => Box::new(no_partial(char(*c).with(value(())))),
        Production::Ref(r) => Box::new(no_partial(parser_for_rule(&rules[r], rules))),
        Production::Seq(r1, r2) => Box::new(no_partial(
            attempt((
                parser_for_rule(&rules[r1], rules),
                parser_for_rule(&rules[r2], rules),
            ))
            .with(value(())),
        )),
        Production::SeqN(ps) => ps
            .iter()
            .map(|r| parser_for_rule(&rules[&r], rules))
            .fold(Box::new(value(())), |parser, rule| {
                Box::new(no_partial(parser.skip(rule)))
            }),
        Production::Plus(r) => Box::new(no_partial(
            skip_many1(attempt(parser_for_rule(&rules[r], rules))).with(value(())),
        )),
        Production::Or(p1, p2) => Box::new(no_partial(
            Parser::or(
                attempt(parser_for_rule(&p1, rules)),
                attempt(parser_for_rule(&p2, rules)),
            )
            .with(value(())),
        )),
        Production::OrN(ps) => {
            let mut parsers = ps
                .iter()
                .map(|p| Box::new(no_partial(attempt(parser_for_rule(p, rules)))));
            let first = parsers.next().unwrap();

            parsers.fold(first, |parser, rule| {
                Box::new(no_partial(attempt(parser.or(rule)).with(value(()))))
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_plus() {
        let mut rules = HashMap::new();
        rules.insert(0, Production::Char('a'));

        let mut p = parser_for_rule(&Production::Plus(0), &rules);

        for input in &["a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa"] {
            assert!(matches!(p.parse(*input), Ok(((), ""))));
        }
    }

    #[test]
    fn test_seq_n() {
        let mut rules = HashMap::new();
        rules.insert(1, Production::Char('a'));
        rules.insert(2, Production::Char('b'));

        for input in &["a", "aa", "aaa", "aaaa", "aaaaa", "aaaaaa"] {
            let len = input.len();
            let mut p = parser_for_rule(&Production::SeqN(vec![1; len]), &rules);

            println!("input = {:?}", input);
            assert!(matches!(p.parse(*input), Ok(((), ""))));
        }
        for input in &["b", "bb", "bbb", "bbbb", "bbbbb", "bbbbbb"] {
            let len = input.len();
            let mut p = parser_for_rule(&Production::SeqN(vec![2; len]), &rules);

            assert!(matches!(p.parse(*input), Ok(((), ""))));
        }
    }

    #[test]
    fn test_or_n() {
        {
            let rules = HashMap::new();
            let rule = Production::OrN(('a'..='z').map(|c| Production::Char(c)).collect());

            for c in 'a'..='z' {
                let input = {
                    let mut buf = String::with_capacity(1);
                    buf.push(c);
                    buf
                };

                let mut p = parser_for_rule(&rule, &rules);
                assert!(matches!(p.parse(input.as_str()), Ok(((), ""))));
            }
        }

        {
            let mut ps = vec![];

            let mut rules = HashMap::new();
            for c in 'a'..='z' {
                rules.insert(c as usize, Production::Char(c));
                ps.push(Production::Seq(c as usize, c as usize));
            }

            let rule = Production::OrN(ps);

            for c in 'a'..='z' {
                let input = {
                    let mut buf = String::with_capacity(2);
                    buf.push(c);
                    buf.push(c);
                    buf
                };
                let mut p = parser_for_rule(&rule, &rules);

                assert!(matches!(p.parse(input.as_str()), Ok(((), ""))));
            }
        }

        {
            let mut rules = HashMap::new();
            rules.insert('a' as usize, Production::Char('a'));
            rules.insert('b' as usize, Production::Char('b'));
            rules.insert('z' as usize, Production::Char('z'));

            // aa | ab | bb
            rules.insert(
                1,
                Production::OrN(vec![
                    Production::Seq('a' as usize, 'a' as usize),
                    Production::Seq('a' as usize, 'b' as usize),
                    Production::Seq('b' as usize, 'b' as usize),
                ]),
            );

            rules.insert(0, Production::Plus(1));

            let rule = Production::Seq(0, 'z' as usize);

            for input in &[
                "aaz", "abz", "bbz", "aaaaz", "aaabz", "aabbz", "abaaz", "ababz", "abbbz", "bbaaz",
                "bbabz", "bbbbz",
            ] {
                let mut p = parser_for_rule(&rule, &rules);
                assert!(matches!(p.parse(*input), Ok(((), ""))));
            }
        }

        {
            let mut rules = HashMap::new();
            rules.insert('a' as usize, Production::Char('a'));
            rules.insert('b' as usize, Production::Char('b'));
            rules.insert('c' as usize, Production::Char('c'));
            rules.insert('d' as usize, Production::Char('d'));
            rules.insert('z' as usize, Production::Char('z'));

            rules.insert(
                0,
                Production::OrN(vec![
                    Production::SeqN(vec!['a' as usize, 'b' as usize]),
                    Production::SeqN(vec!['a' as usize, 'a' as usize, 'b' as usize, 'b' as usize]),
                    Production::SeqN(vec![
                        'a' as usize,
                        'a' as usize,
                        'a' as usize,
                        'b' as usize,
                        'b' as usize,
                        'b' as usize,
                    ]),
                ]),
            );

            rules.insert(
                1,
                Production::Or(
                    Box::new(Production::Ref('c' as usize)),
                    Box::new(Production::Ref('d' as usize)),
                ),
            );

            let rule = Production::SeqN(vec![0, 1, 'z' as usize]);

            for input in &["abcz", "abdz"] {
                let mut p = parser_for_rule(&rule, &rules);
                assert!(matches!(p.parse(*input), Ok(((), ""))));
            }
        }
    }
}
