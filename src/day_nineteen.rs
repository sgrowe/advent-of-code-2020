use super::utils::{get_text_up_to, start_day};
use std::collections::HashMap;

pub fn main() {
    let input = start_day("nineteen");

    let mut lines = input.lines();

    let rules = parse_rules(&mut lines);

    println!("Part one: {}", part_one(&rules, lines));
    // println!("Part two: {}", part_two(&input));
    println!();
}

fn part_one<'a, I>(rules: &Rules, messages: I) -> usize
where
    I: Iterator<Item = &'a str>,
{
    messages.filter(|m| rules.is_valid(m)).count()
}

#[derive(Debug, Copy, Clone)]
enum Rule {
    Letter(char),
    Single(u8),
    Two([u8; 2]),
    Three([u8; 3]),
    EitherSingle(u8, u8),
    Either([u8; 2], [u8; 2]),
}

impl Rule {
    pub fn parse(input: &str) -> Self {
        let mut parts = input.split_ascii_whitespace();

        let a = parts.next().unwrap();

        if a.starts_with('"') {
            return Rule::Letter(a.chars().nth(1).unwrap());
        }

        let a = parse_int(a);

        let b = match parts.next() {
            Some("|") => {
                let b = parse_int(parts.next().unwrap());
                return Rule::EitherSingle(a, b);
            }
            Some(b) => parse_int(b),
            None => return Rule::Single(a),
        };

        match parts.next() {
            Some("|") => {
                let c = parse_int(parts.next().unwrap());
                let d = parse_int(parts.next().unwrap());

                Rule::Either([a, b], [c, d])
            }
            Some(x) => Rule::Three([a, b, parse_int(x)]),
            None => Rule::Two([a, b]),
        }
    }
}

fn parse_int(s: &str) -> u8 {
    s.parse().unwrap()
}

fn parse_rules<'a, I>(lines: &mut I) -> Rules
where
    I: Iterator<Item = &'a str>,
{
    let mut rules = HashMap::new();

    for line in lines {
        if line.is_empty() {
            break;
        }

        let (num, rest) = get_text_up_to(line, ':');
        let num = parse_int(num);
        let rule = Rule::parse(rest.trim());

        rules.insert(num, rule);
    }

    Rules(rules)
}

#[derive(Debug)]
struct Rules(HashMap<u8, Rule>);

impl Rules {
    pub fn is_valid(&self, msg: &str) -> bool {
        let len = self.match_length(msg, 0, true);

        len == Some(msg.len())
    }

    pub fn match_length(&self, msg: &str, rule_id: u8, is_end: bool) -> Option<usize> {
        let rule = self.0[&rule_id];

        let res = match rule {
            Rule::Letter(c) => {
                if msg.chars().next()? == c {
                    if !is_end || msg.len() == 1 {
                        Some(1)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Rule::Single(id) => self.match_length(msg, id, is_end),
            Rule::Two(rules) => self.match_both(msg, rules, is_end),
            Rule::Three([a, b, c]) => {
                let mut i = self.match_length(msg, a, false)?;

                i += self.match_length(&msg[i..], b, false)?;

                i += self.match_length(&msg[i..], c, is_end)?;

                Some(i)
            }
            Rule::EitherSingle(a, b) => self
                .match_length(msg, a, is_end)
                .or_else(|| self.match_length(msg, b, is_end)),
            Rule::Either(pair_a, pair_b) => {
                let x = self.match_both(msg, pair_a, is_end);

                x.or_else(|| self.match_both(msg, pair_b, is_end))
            }
        };

        res
    }

    fn match_both(&self, msg: &str, [a, b]: [u8; 2], is_end: bool) -> Option<usize> {
        let mut i = self.match_length(msg, a, false)?;
        i += self.match_length(&msg[i..], b, is_end)?;

        Some(i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE_ONE: &str = "
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb
";

    #[test_case("ababbb", true)]
    #[test_case("abbbab", true)]
    #[test_case("bababa", false)]
    #[test_case("aaabbb", false)]
    #[test_case("aaaabbb", false)]
    fn sample_input_part_one(msg: &str, is_valid: bool) {
        let rules = parse_rules(&mut EXAMPLE_ONE.trim().lines());

        assert_eq!(rules.is_valid(msg), is_valid);
    }
}
