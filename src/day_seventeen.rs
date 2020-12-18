use super::utils::{first, get_text_up_to, start_day};
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

pub fn main() {
    let input = start_day("seventeen");

    let (rules, my_ticket, nearby_tickets) = parse_input(&input);

    println!("Part one: {}", part_one(&rules, &nearby_tickets));
    println!(
        "Part two: {}",
        part_two(&rules, &my_ticket, &nearby_tickets)
    );
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

fn part_two(rules: &FieldRules, ticket: &Ticket, nearby_tickets: &[Ticket]) -> usize {
    let valid_tickets = nearby_tickets.iter().filter(|t| t.is_valid(&rules));

    let fields = rules.fields_in_right_order(valid_tickets);

    let mut sum = 1;

    for (i, field) in fields.iter().enumerate() {
        if field.starts_with("departure ") {
            sum *= ticket.0[i];
        }
    }

    sum
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

            (field_name, [first, second])
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

    pub fn fields_in_right_order<'b, I>(&self, valid_tickets: I) -> Vec<&'a str>
    where
        I: Iterator<Item = &'b Ticket>,
    {
        let num_fields = self.0.len();

        let mut possible_positions: HashMap<&str, HashSet<usize>> = self
            .0
            .keys()
            .map(|&field| (field, (0..num_fields).collect()))
            .collect();

        for ticket in valid_tickets {
            for (i, x) in ticket.0.iter().enumerate() {
                for (field, [range_a, range_b]) in &self.0 {
                    if !range_a.contains(x) && !range_b.contains(x) {
                        let positions = possible_positions.get_mut(field).unwrap();

                        if positions.remove(&i) && positions.len() == 1 {
                            assert!(!positions.is_empty());

                            let sole_value = *first(positions.iter());

                            clear(&mut possible_positions, sole_value, field);
                        }
                    }
                }
            }
        }

        let mut fields: Vec<(&str, usize)> = possible_positions
            .iter()
            .map(|(&f, positions)| {
                assert!(!positions.is_empty());
                (f, *first(positions))
            })
            .collect();

        fields.sort_by_key(|(_, pos)| *pos);

        fields.into_iter().map(|(f, _)| f).collect()
    }
}

fn clear(
    mut possible_positions: &mut HashMap<&str, HashSet<usize>>,
    value: usize,
    excluded_field: &str,
) {
    let fields: Vec<&str> = possible_positions.keys().copied().collect();

    for f in fields {
        if f == excluded_field {
            continue;
        }

        let positions = possible_positions.get_mut(f).unwrap();

        if positions.remove(&value) && positions.len() == 1 {
            assert!(!positions.is_empty());

            let value = *first(positions.iter());

            clear(&mut possible_positions, value, f);
        }
    }
}

#[derive(Debug)]
struct Ticket(Vec<usize>);

impl Ticket {
    pub fn parse(s: &str) -> Self {
        Ticket(s.split(',').map(|s| s.parse().unwrap()).collect())
    }

    pub fn first_invalid_field(&self, rules: &FieldRules) -> Option<usize> {
        self.0.iter().find(|&v| !rules.is_valid_value(*v)).copied()
    }

    pub fn is_valid(&self, rules: &FieldRules) -> bool {
        self.first_invalid_field(rules).is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_input_file;

    #[test]
    fn sample_input_part_one() {
        let input = "
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
"
        .trim();

        let (rules, _, nearby_tickets) = parse_input(input);

        assert_eq!(part_one(&rules, &nearby_tickets), 71);
    }

    const PART_TWO_SAMPLE: &str = "
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
";

    #[test]
    fn sample_input_fields_in_right_order() {
        let (rules, _, nearby_tickets) = parse_input(PART_TWO_SAMPLE.trim());

        let valid_tickets = nearby_tickets.iter().filter(|t| t.is_valid(&rules));

        assert_eq!(
            rules.fields_in_right_order(valid_tickets),
            vec!["row", "class", "seat"]
        );
    }

    #[test]
    fn sample_input_part_two() {
        let (rules, my_ticket, nearby_tickets) = parse_input(PART_TWO_SAMPLE.trim());

        assert_eq!(part_two(&rules, &my_ticket, &nearby_tickets), 1);
    }

    #[test]
    fn part_two_answer() {
        let input = read_input_file("seventeen");

        let (rules, my_ticket, nearby_tickets) = parse_input(&input);

        assert_eq!(part_two(&rules, &my_ticket, &nearby_tickets), 2355350878831);
    }
}
