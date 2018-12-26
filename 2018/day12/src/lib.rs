use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

use failure::{format_err, Error, ResultExt};

#[derive(Clone, Copy, Debug, Default)]
pub struct Rule {
    input: [bool; 5],
    output: bool,
}

#[derive(Debug)]
pub struct Garden {
    rules: [bool; 32],
    pub state: VecDeque<bool>,
    pub offset: usize,
}

impl Garden {
    pub fn new(rules: [Rule; 32], state: Vec<bool>) -> Self {
        let mut sorted_rules = [false; 32];
        for rule in &rules {
            sorted_rules[Garden::key_from_slice(&rule.input)] = rule.output;
        }

        let mut state = state.into_iter().collect();
        let offset = Garden::extend(&mut state);

        Garden {
            rules: sorted_rules,
            state,
            offset,
        }
    }

    fn with_offset(rules: [Rule; 32], state: Vec<bool>, offset: usize) -> Self {
        let mut g = Garden::new(rules, state);
        g.offset += offset;
        g
    }

    pub fn pots<'a>(&'a self) -> impl Iterator<Item = (i32, bool)> + 'a {
        (-(self.offset as i32)..).zip(self.state.iter().map(|&b| b))
    }

    // Ensure there are at least 4 empty pots on each side.
    fn extend(state: &mut VecDeque<bool>) -> usize {
        if let Some(n) = state.iter().rev().position(|x| *x) {
            if n < 4 {
                for _ in 0..(4 - n) {
                    state.push_back(false);
                }
            }
        }

        if let Some(n) = state.iter().position(|x| *x) {
            if n < 4 {
                for _ in 0..(4 - n) {
                    state.push_front(false);
                }

                return 4 - n;
            }
        }

        0
    }

    fn key_from_slice(t: &[bool]) -> usize {
        assert_eq!(t.len(), 5);

        Garden::key((t[0], t[1], t[2], t[3], t[4]))
    }

    fn key(t: (bool, bool, bool, bool, bool)) -> usize {
        t.4 as usize
            | (t.3 as usize) << 1
            | (t.2 as usize) << 2
            | (t.1 as usize) << 3
            | (t.0 as usize) << 4
    }

    pub fn next(&mut self) {
        let mut new_state = self
            .state
            .iter()
            .skip(2)
            .take(self.state.len() - 2)
            .scan(
                vec![false, false, false, false, false]
                    .into_iter()
                    .collect::<VecDeque<_>>(),
                |state, &pot| {
                    state.pop_front();
                    state.push_back(pot);

                    Some(
                        self.rules[Garden::key((state[0], state[1], state[2], state[3], state[4]))],
                    )
                },
            )
            .collect::<VecDeque<_>>();

        let delta_offset = Garden::extend(&mut new_state);

        self.state = new_state;
        self.offset += delta_offset;
    }

    pub fn to_string(&self) -> String {
        let first = self.state.iter().position(|&p| p).unwrap() - 4;
        let last = self.state.iter().rposition(|&p| p).unwrap() + 4;

        self.state
            .iter()
            .skip(first)
            .take(last - first + 1)
            .map(|p| match p {
                true => '#',
                false => '.',
            })
            .collect()
    }

    pub fn score(&self) -> i32 {
        self.pots()
            .filter_map(|(i, pot)| if pot { Some(i) } else { None })
            .sum()
    }
}

pub fn read_input() -> Result<Garden, Error> {
    let mut f =
        File::open("input").with_context(|e| format_err!("Could not open input file: {}", e))?;

    let buf = {
        let mut buf = String::new();
        f.read_to_string(&mut buf)
            .with_context(|e| format_err!("Could not read input file: {}", e))?;

        buf
    };

    parser::parse_input(&buf)
}

mod parser {
    use combine::parser::char::{space, string};
    use combine::parser::item::{eof, satisfy};
    use combine::parser::repeat::{count_min_max, many1, skip_many1};
    use combine::stream::state::State;
    use combine::{ParseError, Parser, Stream};
    use failure::{format_err, Error};

    use super::{Garden, Rule};

    fn plant<I>() -> impl Parser<Input = I, Output = bool>
    where
        I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        satisfy(|c| c == '.' || c == '#').map(|c| match c {
            '.' => false,
            '#' => true,
            _ => unreachable!(),
        })
    }

    pub fn rule<I>() -> impl Parser<Input = I, Output = Rule>
    where
        I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        (
            count_min_max::<Vec<_>, _>(5, 5, plant()).skip(string(" => ")),
            plant(),
        )
            .map(|(inputs, output)| Rule {
                input: [inputs[0], inputs[1], inputs[2], inputs[3], inputs[4]],
                output,
            })
    }

    pub fn parse_rule(s: &str) -> Result<Rule, Error> {
        rule()
            .skip(eof())
            .easy_parse(s)
            .map(|(rule, _)| rule)
            .map_err(|e| format_err!("Could not parse rule `{}': {}", s, e))
    }

    fn state<I>() -> impl Parser<Input = I, Output = Vec<bool>>
    where
        I: Stream<Item = char>,
        I::Error: ParseError<I::Item, I::Range, I::Position>,
    {
        many1::<Vec<_>, _>(plant())
    }

    pub fn parse_state(s: &str) -> Result<Vec<bool>, Error> {
        state()
            .skip(eof())
            .easy_parse(s)
            .map(|(st, _)| st)
            .map_err(|e| format_err!("Could not parse state `{}': {}", s, e))
    }

    pub fn parse_input(s: &str) -> Result<Garden, Error> {
        string("initial state: ")
            .with((
                state(),
                skip_many1(space()),
                many1::<Vec<_>, _>(rule().skip(space())),
            ))
            .skip(eof())
            .map(|(state, (), rules_vec)| {
                let mut rules = [Rule::default(); 32];
                for rule in rules_vec {
                    rules[Garden::key_from_slice(&rule.input)] = rule;
                }
                Garden::new(rules, state)
            })
            .easy_parse(State::new(s))
            .map(|(r, _)| r)
            .map_err(|e| format_err!("Could not parse input file: {}", e))
    }
}

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;

    use super::{parser, Garden, Rule};

    lazy_static! {
        static ref RULES: [Rule; 32] = {
            let rule_strs = [
                "...## => #",
                "..#.. => #",
                ".#... => #",
                ".#.#. => #",
                ".#.## => #",
                ".##.. => #",
                ".#### => #",
                "#.#.# => #",
                "#.### => #",
                "##.#. => #",
                "##.## => #",
                "###.. => #",
                "###.# => #",
                "####. => #",
            ];

            let mut rules = [Rule::default(); 32];

            for s in rule_strs.iter() {
                let rule = parser::parse_rule(s).unwrap();
                rules[Garden::key_from_slice(&rule.input[..])] = rule;
            }

            rules
        };
        static ref STATES: Vec<Vec<bool>> = {
            let states = vec![
                "...#..#.#..##......###...###...........",
                "...#...#....#.....#..#..#..#...........",
                "...##..##...##....#..#..#..##..........",
                "..#.#...#..#.#....#..#..#...#..........",
                "...#.#..#...#.#...#..#..##..##.........",
                "....#...##...#.#..#..#...#...#.........",
                "....##.#.#....#...#..##..##..##........",
                "...#..###.#...##..#...#...#...#........",
                "...#....##.#.#.#..##..##..##..##.......",
                "...##..#..#####....#...#...#...#.......",
                "..#.#..#...#.##....##..##..##..##......",
                "...#...##...#.#...#.#...#...#...#......",
                "...##.#.#....#.#...#.#..##..##..##.....",
                "..#..###.#....#.#...#....#...#...#.....",
                "..#....##.#....#.#..##...##..##..##....",
                "..##..#..#.#....#....#..#.#...#...#....",
                ".#.#..#...#.#...##...#...#.#..##..##...",
                "..#...##...#.#.#.#...##...#....#...#...",
                "..##.#.#....#####.#.#.#...##...##..##..",
                ".#..###.#..#.#.#######.#.#.#..#.#...#..",
                ".#....##....#####...#######....#.#..##.",
            ];

            states
                .into_iter()
                .map(|st| parser::parse_state(st).unwrap())
                .collect()
        };
    }

    #[test]
    fn test_garden_key() {
        assert_eq!(Garden::key((false, false, false, false, false)), 0);
        assert_eq!(Garden::key((false, false, false, false, true)), 1);
        assert_eq!(Garden::key((true, false, false, false, false)), 16);
        assert_eq!(Garden::key((true, true, true, true, true)), 31);
    }

    #[test]
    fn test_garden_extend() {
        {
            let mut st = vec![false, false, false, false, true, false, false, false]
                .into_iter()
                .collect();

            assert_eq!(Garden::extend(&mut st), 0);
            assert_eq!(
                st,
                vec![false, false, false, false, true, false, false, false, false]
            );
        }

        {
            let mut st = vec![false, false, false, true, false, false, false]
                .into_iter()
                .collect();

            assert_eq!(Garden::extend(&mut st), 1);
            assert_eq!(
                st,
                vec![false, false, false, false, true, false, false, false, false]
            );
        }

        {
            let mut st = vec![false, false, true, false, false].into_iter().collect();

            assert_eq!(Garden::extend(&mut st), 2);
            assert_eq!(
                st,
                vec![false, false, false, false, true, false, false, false, false]
            );
        }

        {
            let mut st = vec![false, true, true, true, false].into_iter().collect();
            assert_eq!(Garden::extend(&mut st), 3);

            assert_eq!(
                st,
                vec![false, false, false, false, true, true, true, false, false, false, false]
            );
        }

        {
            let mut st = vec![true, false, false, false, true].into_iter().collect();

            assert_eq!(Garden::extend(&mut st), 4);

            assert_eq!(
                st,
                vec![
                    false, false, false, false, true, false, false, false, true, false, false,
                    false, false
                ]
            );
        }
    }

    #[test]
    fn test_to_string() {
        assert_eq!(
            Garden::with_offset(RULES.clone(), STATES[0].clone(), 3).to_string(),
            "....#..#.#..##......###...###...."
        );

        assert_eq!(
            Garden::with_offset(RULES.clone(), STATES[1].clone(), 3).to_string(),
            "....#...#....#.....#..#..#..#...."
        );
    }

    #[test]
    fn test_next() {
        let mut garden = Garden::with_offset(RULES.clone(), STATES[0].clone(), 3);

        let expected_strs = vec![
            "....#..#.#..##......###...###....",
            "....#...#....#.....#..#..#..#....",
            "....##..##...##....#..#..#..##....",
            "....#.#...#..#.#....#..#..#...#....",
            "....#.#..#...#.#...#..#..##..##....",
            "....#...##...#.#..#..#...#...#....",
            "....##.#.#....#...#..##..##..##....",
            "....#..###.#...##..#...#...#...#....",
            "....#....##.#.#.#..##..##..##..##....",
            "....##..#..#####....#...#...#...#....",
            "....#.#..#...#.##....##..##..##..##....",
            "....#...##...#.#...#.#...#...#...#....",
            "....##.#.#....#.#...#.#..##..##..##....",
            "....#..###.#....#.#...#....#...#...#....",
            "....#....##.#....#.#..##...##..##..##....",
            "....##..#..#.#....#....#..#.#...#...#....",
            "....#.#..#...#.#...##...#...#.#..##..##....",
            "....#...##...#.#.#.#...##...#....#...#....",
            "....##.#.#....#####.#.#.#...##...##..##....",
            "....#..###.#..#.#.#######.#.#.#..#.#...#....",
            "....#....##....#####...#######....#.#..##....",
        ];

        let expected_scores = vec![
            145, 91, 132, 102, 154, 115, 174, 126, 213, 138, 213, 136, 218, 133, 235, 149, 226,
            170, 280, 287, 325,
        ];

        for i in 0..=20 {
            assert_eq!(garden.to_string(), expected_strs[i]);
            assert_eq!(garden.score(), expected_scores[i]);

            garden.next();
        }
    }

    #[test]
    fn test_score() {
        let garden = Garden::with_offset(RULES.clone(), STATES[20].clone(), 3);

        assert_eq!(garden.score(), 325);
    }

}
