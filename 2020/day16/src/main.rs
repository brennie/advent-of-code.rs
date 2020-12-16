use std::collections::{HashMap, HashSet};
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
    let valid_tickets = nearby_tickets
        .iter()
        .filter(|t| t.valid(ruleset))
        .cloned()
        .collect::<Vec<_>>();

    let fields = {
        let mut fields = ruleset.keys().map(String::from).collect::<Vec<_>>();
        fields.sort();
        fields
    };

    let mut solution = Vec::<&str>::new();
    let mut i = 0;

    let mut invalid_assignments = HashMap::<String, HashSet<usize>>::new();
    while i < fields.len() {
        let mut backtrack = false;
        if solution.len() == i {
            // We have reached a previously un-assigned field.
            solution.push(&fields[0]);
        } else {
            // We are re-assigning a field that failed.
            let j = fields.iter().position(|f| f == solution[i]).unwrap() + 1;

            if j >= fields.len() {
                // We have eliminated all possible assignments for this index,
                // so we must backtrack to a previous assignment.
                solution.pop().unwrap();
                backtrack = true;

                if i == 0 {
                    panic!();
                } else {
                    i -= 1;
                }
            } else {
                solution[i] = &fields[j];
            }
        }

        if !backtrack && check(ruleset, &valid_tickets, &solution, &mut invalid_assignments) {
            i += 1;
        }
    }

    println!("{:?}", solution);

    let mut product = 1;
    for (i, field) in solution.iter().enumerate() {
        if !field.starts_with("departure") {
            continue;
        }

        product *= ticket.0[i];
    }

    product
}

fn check(
    ruleset: &RuleSet,
    tickets: &[Ticket],
    solution: &[&str],
    invalid_assignments: &mut HashMap<String, HashSet<usize>>,
) -> bool {
    for (i, a) in solution.iter().enumerate() {
        for (j, b) in solution.iter().enumerate() {
            if i == j {
                continue;
            }

            if a == b {
                return false;
            }
        }
    }

    // We only need to check the last field!
    let i = solution.len() - 1;
    let field = solution[i];

    if let Some(invalid) = invalid_assignments.get(field) {
        if invalid.contains(&i) {
            return false;
        }
    }

    let rule = ruleset.get(&*field).unwrap();
    for ticket in tickets {
        let value = ticket.0[i];

        if !rule.valid(value) {
            invalid_assignments
                .entry(String::from(field))
                .or_default()
                .insert(i);

            return false;
        }
    }

    true
}
