use super::utils::start_day;
use crate::utils::get_text_up_to;
use std::collections::HashMap;
use std::ops::RangeInclusive;

pub fn main() {
    let input = start_day("seventeen");

    let (rules, my_ticket, nearby_tickets) = parse_input(&input);

    println!("Part one: {}", part_one(&rules, &nearby_tickets));
    // println!("Part two: {}", part_two(&map));
    println!();
}

fn part_one(rules: &FieldRules, nearby_tickets: &[Ticket]) -> usize {
    let mut error_rate = 0;

    for Ticket(fields) in nearby_tickets {
        for &value in fields {
            if !rules.is_valid_value(value) {
                error_rate += value;
                break;
            }
        }
    }

    error_rate
}

fn parse_input(input: &str) -> (FieldRules, Ticket, Vec<Ticket>) {
    let mut lines = input.lines();

    let rules = lines
        .by_ref()
        .take_while(|&l| l != "")
        .map(|l| {
            let (field_name, rest) = get_text_up_to(l, ':');

            let mut ranges = rest.trim().split(" or ");

            let first = parse_range(ranges.next().unwrap());
            let second = parse_range(ranges.next().unwrap());

            dbg!(field_name, [first, second])
        })
        .collect();

    assert_eq!(lines.next().unwrap(), "your ticket:");
    let my_ticket = Ticket::parse(lines.next().unwrap());
    lines.next().unwrap();

    assert_eq!(lines.next().unwrap(), "nearby tickets:");
    let nearby_tickets = lines.map(Ticket::parse).collect();

    (FieldRules(rules), my_ticket, nearby_tickets)
}

fn parse_range(s: &str) -> RangeInclusive<usize> {
    let (start, end) = get_text_up_to(s, '-');

    RangeInclusive::new(start.parse().unwrap(), end.parse().unwrap())
}

#[derive(Debug)]
struct FieldRules<'a>(HashMap<&'a str, [RangeInclusive<usize>; 2]>);

impl<'a> FieldRules<'a> {
    pub fn is_valid_value(&self, x: usize) -> bool {
        self.0
            .values()
            .any(|[a, b]| a.contains(&x) || b.contains(&x))
    }
}

#[derive(Debug)]
struct Ticket(Vec<usize>);

impl Ticket {
    pub fn parse(s: &str) -> Self {
        Ticket(s.split(',').map(|s| s.parse().unwrap()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";

    #[test]
    fn sample_input_part_one() {
        let (rules, _, nearby_tickets) = parse_input(TEST_INPUT.trim());

        assert_eq!(part_one(&rules, &nearby_tickets), 71);
    }
}
