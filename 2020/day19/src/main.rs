use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::Result;
use combine::parser::char::{char, digit, space};
use combine::parser::choice::optional;
use combine::parser::combinator::attempt;
use combine::parser::range::{range, recognize};
use combine::parser::repeat::skip_many1;
use combine::parser::token::any;
use combine::{ParseError, Parser, RangeStream};
use itertools::Itertools;
use regex::{Regex, RegexBuilder};

fn main() -> Result<()> {
    let (rules, messages) = parse_input()?;

    {
        let mut rules = rules.clone();
        let production = collapse(&mut rules);
        let re = to_regex(&production);

        let count = messages.iter().filter(|line| re.is_match(line)).count();

        println!("part 1: {}", count);
    }

    {
        let max_len = messages.iter().map(String::len).max().unwrap();
        let mut rules = rules.clone();
        rules.insert(8, plus(&Production::Ref(42), max_len));
        rules.insert(
            11,
            balanced(&Production::Ref(42), &Production::Ref(31), max_len / 2),
        );

        let production = collapse(&mut rules);
        let re = to_regex(&production);

        let count = messages.iter().filter(|line| re.is_match(line)).count();

        println!("part 2: {}", count);
    }

    Ok(())
}

/// Generate a production of the form `p+` up to a max length of `max_depth`.
fn plus(p: &Production, max_depth: usize) -> Production {
    let mut combinations = Vec::with_capacity(max_depth);

    combinations.push(p.clone());

    for i in 1..max_depth {
        let mut seq = Vec::with_capacity(i);
        for _ in 0..i {
            seq.push(p.clone());
        }
        combinations.push(Production::Seq(seq));
    }

    Production::Or(combinations)
}

/// Generate a production of the form (pq|ppqq|pppqqq|...) up to a max length of `max_depth * 2`.
fn balanced(p: &Production, q: &Production, max_depth: usize) -> Production {
    let mut combinations = Vec::with_capacity(max_depth);

    for i in 1..=max_depth {
        let mut seq = Vec::with_capacity(i * 2);

        for _ in 0..i {
            seq.push(p.clone());
        }

        for _ in 0..i {
            seq.push(q.clone());
        }

        combinations.push(Production::Seq(seq));
    }

    Production::Or(combinations)
}

type Rules = HashMap<usize, Production>;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Production {
    Char(char),
    Ref(usize),
    Or(Vec<Production>),
    Seq(Vec<Production>),
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

fn rule<'a, I>() -> impl Parser<I, Output = (usize, Production)> + 'a
where
    I: RangeStream<Token = char, Range = &'a str> + 'a,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    let number = || recognize(skip_many1(digit())).map(|s: &str| s.parse::<usize>().unwrap());
    let string = || char('"').with(any()).skip(char('"')).map(Production::Char);

    let ref_seq = || {
        (number(), optional(attempt(space().with(number())))).map(|(a, b)| match (a, b) {
            (a, Some(b)) => Production::Seq(vec![Production::Ref(a), Production::Ref(b)]),
            (a, None) => Production::Ref(a),
        })
    };

    let ref_seq_or =
        (ref_seq(), optional(range(" | ").with(ref_seq()))).map(|(a, b)| match (a, b) {
            (a, Some(b)) => Production::Or(vec![a, b]),
            (a, None) => a,
        });

    let rule = || Parser::or(string(), ref_seq_or);

    (number().skip(range(": ")), rule())
}

fn reverse(rules: &Rules) -> HashMap<usize, HashSet<usize>> {
    let mut reverse: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (id, production) in rules {
        foreach_production(production, &mut |p| {
            if let Production::Ref(n) = p {
                reverse.entry(*n).or_default().insert(*id);
            }
        });
    }

    reverse
}

fn foreach_production<F>(production: &Production, f: &mut F)
where
    F: FnMut(&Production),
{
    f(production);

    match production {
        Production::Char(..) => {}
        Production::Ref(..) => {}
        Production::Or(ref ps) => {
            for p in ps {
                foreach_production(p, f);
            }
        }
        Production::Seq(ref ps) => {
            for p in ps {
                foreach_production(p, f);
            }
        }
    }
}

fn foreach_production_mut<F>(production: &mut Production, f: &mut F)
where
    F: FnMut(&mut Production),
{
    f(production);

    match production {
        Production::Char(..) => {}
        Production::Ref(..) => {}
        Production::Or(ref mut ps) => {
            for p in ps.iter_mut() {
                foreach_production_mut(p, f);
            }
        }
        Production::Seq(ref mut ps) => {
            for p in ps.iter_mut() {
                foreach_production_mut(p, f);
            }
        }
    }
}

fn collapse(rules: &mut Rules) -> Production {
    let reverse = reverse(rules);

    let mut open: VecDeque<usize> = VecDeque::new();

    // Find rules that are `-> Char` productions only.
    for (id, production) in rules.iter() {
        if let Production::Char(c) = production {
            open.push_back(*id);
        }
    }

    while let Some(id) = open.pop_front() {
        if id == 0 {
            break;
        }

        let production = rules.remove_entry(&id).expect("entry should exist").1;

        // Replace each instance of Ref(id) with this
        for j in reverse
            .get(&id)
            .expect(&format!("each id has entry in reverse graph: {}", id))
            .iter()
        {
            let p = rules.get_mut(&j).unwrap();
            // println!("before {:?}", p);

            foreach_production_mut(p, &mut |sub| {
                if let Production::Ref(k) = sub {
                    if *k == id {
                        *sub = production.clone();
                    }
                }
            });

            // println!("after {:?}\n", p);

            let mut simplified = true;
            foreach_production(&p, &mut |sub| {
                if let Production::Ref(..) = sub {
                    simplified = false;
                }
            });

            if simplified {
                open.push_back(*j);
            }
        }
    }

    let mut rule = rules.remove_entry(&0).unwrap().1;
    assert!(rules.is_empty());

    rule
}

fn to_regex(p: &Production) -> Regex {
    let mut buf = String::from("^");
    buf.push_str(&to_regex_raw(p));
    buf.push('$');
    RegexBuilder::new(&buf)
        .size_limit(10485760 * 1024) // The default limit is much too small.
        .build()
        .unwrap()
}

fn to_regex_raw(p: &Production) -> String {
    match p {
        Production::Char(c) => c.to_string(),
        Production::Ref(..) => panic!("no refs in final production"),
        Production::Seq(ref qs) => qs.iter().map(to_regex_raw).collect::<String>(),
        Production::Or(ref qs) => {
            let mut buffer: String = "(?:".into();

            for q in qs.iter().map(to_regex_raw).intersperse("|".into()) {
                buffer.push_str(&q);
            }
            buffer.push(')');
            buffer
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_regex() {
        assert_eq!(
            to_regex(&Production::Seq(vec![
                Production::Char('a'),
                Production::Char('b'),
                Production::Char('c')
            ]))
            .as_str(),
            "^abc$"
        );
    }

    #[test]
    fn test_plus() {
        assert_eq!(
            plus(&Production::Char('a'), 3),
            Production::Or(vec![
                Production::Char('a'),
                Production::Seq(vec![Production::Char('a'), Production::Char('a')]),
                Production::Seq(vec![
                    Production::Char('a'),
                    Production::Char('a'),
                    Production::Char('a')
                ]),
            ])
        );
    }
}
