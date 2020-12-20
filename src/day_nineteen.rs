use super::utils::{get_text_up_to, start_day};
use std::collections::HashMap;
use tinyvec::ArrayVec;

pub fn main() {
    let input = start_day("nineteen");

    let mut lines = input.lines();

    let rules = parse_rules(&mut lines);

    println!("Part one: {}", part_one(&rules, lines.clone()));
    // println!("Part two: {}", part_two(rules, lines));
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
    Rules(RuleIds),
    Either(RuleIds, RuleIds),
}

type RuleIds = ArrayVec<[u8; 3]>;

impl Rule {
    pub fn parse(input: &str) -> Self {
        let mut parts = input.split_ascii_whitespace();

        let a = parts.next().unwrap();

        if a.starts_with('"') {
            return Rule::Letter(a.chars().nth(1).unwrap());
        }

        let mut rules = ArrayVec::new();

        rules.push(parse_int(a));

        while let Some(rule) = parts.next() {
            if rule == "|" {
                let mut other_rules = ArrayVec::new();

                for rule in parts {
                    other_rules.push(parse_int(rule));
                }

                return Rule::Either(rules, other_rules);
            }

            rules.push(parse_int(rule))
        }

        Rule::Rules(rules)
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
            Rule::Rules(rules) => self.match_many(msg, rules, is_end),

            Rule::Either(rules_a, rules_b) => self
                .match_many(msg, rules_a, is_end)
                .or_else(|| self.match_many(msg, rules_b, is_end)),
        };

        res
    }

    fn match_many(&self, msg: &str, rules: RuleIds, is_end: bool) -> Option<usize> {
        let mut x = self.match_length(msg, rules[0], false)?;

        for (i, &rule) in rules.iter().enumerate().skip(1) {
            let is_last = i == (rules.len() - 1);

            x += self.match_length(&msg[x..], rule, is_last && is_end)?;
        }

        Some(x)
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
