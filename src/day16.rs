use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Input {
    nearby_tickets: Vec<Vec<i32>>,
    my_ticket: Vec<i32>,
    rules: HashMap<String, Vec<RangeInclusive<i32>>>,
}

impl Input {
    fn sum_invalid(&self) -> i32 {
        self.nearby_tickets
            .iter()
            .flatten()
            .filter(|num| self.is_invalid(num))
            .sum()
    }

    fn is_invalid(&self, num: &&i32) -> bool {
        self.rules
            .values()
            .all(|rules| !rules.iter().any(|rule| rule.contains(num)))
    }

    fn kill_invalid(&mut self) {
        let rules = self.rules.clone();
        self.nearby_tickets.retain(|v| {
            v.iter().all(|num| {
                !rules
                    .values()
                    .all(|rules| !rules.iter().any(|rule| rule.contains(num)))
            })
        })
    }

    fn map_index_to_rule(&self) -> u128 {
        let mut possibilities = Vec::new();
        let mut covered = HashSet::new();
        for i in 0..self.my_ticket.len() {
            possibilities.insert(i, self.rules.clone());
        }
        loop {
            // should only be two loops
            for ticket in &self.nearby_tickets {
                for field_index in 0..ticket.len() {
                    if let Some(possible_rules) = possibilities.get_mut(field_index) {
                        let field = ticket[field_index];

                        if possible_rules.len() > 1 {
                            Input::remove_impossible_rules(&mut covered, possible_rules, &field);
                        }

                        if possible_rules.len() == 1 {
                            let guaranteed = possible_rules.iter().next().unwrap();
                            covered.insert(guaranteed.0.clone());
                        }

                        if possibilities.iter().all(|v| v.len() == 1) {
                            return possibilities
                                .iter()
                                .enumerate()
                                .filter(|(_, r)| r.iter().next().unwrap().0.contains("departure"))
                                .fold(1, |acc, (i, _)| acc * self.my_ticket[i] as u128);
                        }
                    }
                }
            }
        }
    }

    fn remove_impossible_rules(
        covered: &mut HashSet<String>,
        possible_rules: &mut HashMap<String, Vec<RangeInclusive<i32>>>,
        field: &i32,
    ) {
        possible_rules.retain(|name, rules| {
            rules.iter().any(|rule| rule.contains(&field)) && !covered.contains(name)
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::day16::Input;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::ops::RangeInclusive;

    #[test]
    fn test_parse() {
        parse("inputs/day16small.txt");
    }

    #[test]
    fn test_num_invalid() {
        assert_eq!(71, parse("inputs/day16small.txt").sum_invalid());
    }

    #[test]
    fn test_p1() {
        assert_eq!(21978, parse("inputs/day16.txt").sum_invalid());
    }

    #[test]
    fn test_p2_small() {
        let mut valid = parse("inputs/day16small2.txt");
        valid.kill_invalid();
        println!("{}", valid.map_index_to_rule());
    }

    #[test]
    fn test_p2() {
        let mut valid = parse("inputs/day16.txt");
        timed!(
            {
                valid.kill_invalid();
                assert_eq!(1053686852011, valid.map_index_to_rule())
            },
            "test_p2"
        );
    }

    #[derive(Eq, PartialEq, Copy, Clone)]
    enum ParseStatus {
        Rules,
        MyTicket,
        NearbyTickets,
    }

    fn parse(path: &str) -> Input {
        let mut status = ParseStatus::Rules;
        let mut nearby_tickets = Vec::new();
        let mut my_ticket = Vec::new();
        let mut rules = HashMap::new();
        for l in BufReader::new(File::open(path).unwrap()).lines() {
            let line = l.unwrap();
            if change_status(&mut status, &line) || line.is_empty() {
                continue;
            }
            match status {
                ParseStatus::Rules => {
                    if let Some((k, rest)) = line.split_once(": ") {
                        let v = rest
                            .split("or")
                            .map(|s| {
                                let (lo, hi) = s.split_once('-').unwrap();
                                RangeInclusive::new(
                                    lo.trim().parse().unwrap(),
                                    hi.trim().parse().unwrap(),
                                )
                            })
                            .collect();
                        rules.insert(String::from(k), v);
                    } else {
                        continue;
                    }
                }
                ParseStatus::MyTicket => {
                    my_ticket = line
                        .split(",")
                        .into_iter()
                        .map(|num| num.trim().parse().unwrap())
                        .collect();
                }
                ParseStatus::NearbyTickets => nearby_tickets.push(
                    line.split(",")
                        .into_iter()
                        .map(|num| num.parse().unwrap())
                        .collect(),
                ),
            }
        }
        Input {
            nearby_tickets,
            my_ticket,
            rules,
        }
    }

    fn change_status(status: &mut ParseStatus, line: &String) -> bool {
        if *status == ParseStatus::Rules && line.contains("your ticket:") {
            *status = ParseStatus::MyTicket;
            true
        } else if *status == ParseStatus::MyTicket && line.contains("nearby tickets:") {
            *status = ParseStatus::NearbyTickets;
            true
        } else {
            false
        }
    }
}
