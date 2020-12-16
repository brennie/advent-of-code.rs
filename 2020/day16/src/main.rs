use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::RangeInclusive;

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let (ruleset, ticket, nearby_tickets) = read_input()?;
    println!("part 1: {}", part1(&ruleset, &ticket, &nearby_tickets));
    println!("part 2: {}", part2(&ruleset, &ticket, &nearby_tickets));

    Ok(())
}

#[derive(Debug)]
struct Rule {
    range1: RangeInclusive<usize>,
    range2: RangeInclusive<usize>,
}

impl Rule {
    fn valid(&self, value: usize) -> bool {
        self.range1.contains(&value) || self.range2.contains(&value)
    }
}

type RuleSet = HashMap<String, Rule>;

#[derive(Clone, Debug)]
struct Ticket(Vec<usize>);

impl Ticket {
    fn valid(&self, ruleset: &RuleSet) -> bool {
        'value: for value in &self.0 {
            for rule in ruleset.values() {
                if rule.valid(*value) {
                    continue 'value;
                }
            }

            return false;
        }

        true
    }
}

fn read_input() -> Result<(RuleSet, Ticket, Vec<Ticket>)> {
    enum State {
        Rules,
        RulesEnd,
        YourTicket,
        YourTicketEnd,
        NearbyTickets,
        Finished,
    }

    let mut state = State::Rules;

    let mut nearby = vec![];
    let mut ruleset = RuleSet::new();

    let mut ticket = Ticket(vec![]);

    let rule_re = Regex::new(r#"^([^:]+): (\d+)-(\d+) or (\d+)-(\d+)$"#).unwrap();

    for line in BufReader::new(File::open("input")?).lines() {
        let line = line?;

        match state {
            State::Rules => {
                if line.len() == 0 {
                    state = State::RulesEnd;
                } else if let Some(m) = rule_re.captures(&line) {
                    let a = m[2].parse()?;
                    let b = m[3].parse()?;
                    let c = m[4].parse()?;
                    let d = m[5].parse()?;

                    ruleset.insert(
                        m[1].to_owned(),
                        Rule {
                            range1: a..=b,
                            range2: c..=d,
                        },
                    );
                } else {
                    panic!();
                }
            }

            State::RulesEnd => {
                if line == "your ticket:" {
                    state = State::YourTicket;
                } else {
                    panic!()
                }
            }

            State::YourTicket => {
                if line.len() == 0 {
                    assert!(ticket.0.len() > 0);
                    state = State::YourTicketEnd;
                } else {
                    for value in line.split(",").map(str::parse) {
                        let value = value?;
                        ticket.0.push(value);
                    }
                }
            }

            State::YourTicketEnd => {
                if line == "nearby tickets:" {
                    state = State::NearbyTickets;
                } else {
                    panic!()
                }
            }

            State::NearbyTickets => {
                if line.len() == 0 {
                    assert!(nearby.len() > 0);
                    state = State::Finished;
                } else {
                    let mut ticket_values = vec![];
                    for value in line.split(",").map(str::parse) {
                        let value = value?;
                        ticket_values.push(value);
                    }
                    nearby.push(Ticket(ticket_values))
                }
            }

            State::Finished => panic!(),
        }
    }

    Ok((ruleset, ticket, nearby))
}

fn part1(ruleset: &RuleSet, _ticket: &Ticket, nearby_tickets: &[Ticket]) -> usize {
    let mut errors = 0;
    for ticket in nearby_tickets {
        'value: for value in &ticket.0 {
            for rule in ruleset.values() {
                if rule.valid(*value) {
                    continue 'value;
                }
            }

            errors += value;
        }
    }

    errors
}

fn part2(ruleset: &RuleSet, ticket: &Ticket, nearby_tickets: &[Ticket]) -> usize {
    let nearby_tickets = nearby_tickets
        .iter()
        .filter(|t| t.valid(ruleset))
        .cloned()
        .collect::<Vec<_>>();

    let fields = {
        let mut fields = ruleset.keys().map(String::from).collect::<Vec<_>>();
        fields.sort();
        fields
    };

    let domains: Vec<Vec<&str>> = (0..ticket.0.len())
        .map(|i| {
            fields
                .iter()
                .filter_map(|field| {
                    let rule = ruleset.get(field).unwrap();
                    for ticket in &nearby_tickets {
                        if !rule.valid(ticket.0[i]) {
                            return None;
                        }
                    }
                    Some(field.as_str())
                })
                .collect()
        })
        .collect();

    let most_constrained: Vec<usize> = {
        let mut indices = (0..ticket.0.len()).collect::<Vec<_>>();
        indices.sort_by_key(|idx| domains[*idx].len());
        indices
    };

    let mut assignment = HashMap::<&str, usize>::new();

    for i in &most_constrained {
        for field in domains[*i].iter() {
            match assignment.entry(field) {
                Entry::Occupied(..) => continue,
                Entry::Vacant(e) => {
                    e.insert(*i);
                    break;
                }
            }
        }
    }

    assignment
        .iter()
        .filter_map(|(field, index)| {
            if field.starts_with("departure") {
                Some(ticket.0[*index])
            } else {
                None
            }
        })
        .product()
}
