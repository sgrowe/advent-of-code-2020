use super::utils::{get_text_up_to, start_day};
use std::collections::HashMap;
use tinyvec::ArrayVec;

pub fn main() {
    let input = start_day("nineteen");

    let mut lines = input.lines();

    let rules = parse_rules(&mut lines);

    println!("Part one: {}", part_one(&rules, lines.clone()));
    println!("Part two: {}", part_two(rules, lines));
    println!();
}

fn part_one<'a, I>(rules: &Rules, messages: I) -> usize
where
    I: Iterator<Item = &'a str>,
{
    messages.filter(|m| rules.is_valid(m)).count()
}

fn part_two<'a, I>(mut rules: Rules, messages: I) -> usize
where
    I: Iterator<Item = &'a str>,
{
    update_rules_8_and_11(&mut rules);

    messages.filter(|m| rules.is_valid(m)).count()
}

fn update_rules_8_and_11(rules: &mut Rules) {
    rules
        .0
        .insert(8, Rule::Either(array_vec!(42), array_vec!(42, 8)));

    rules
        .0
        .insert(11, Rule::Either(array_vec!(42, 31), array_vec!(42, 11, 31)));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

        let is_repeating_special_case = rule_id == 0
            && self.0.get(&8) == Some(&Rule::Either(array_vec!(42), array_vec!(42, 8)))
            && self.0.get(&11) == Some(&Rule::Either(array_vec!(42, 31), array_vec!(42, 11, 31)));

        if is_repeating_special_case {
            let mut i = self.match_length(msg, 42, false)?;
            i += self.match_length(&msg[i..], 42, false)?;

            let mut max_num_31s = 1;

            loop {
                if let Some(x) = self.matches_repeating_31s(&msg[i..], max_num_31s) {
                    return Some(i + x);
                }

                i += self.match_length(&msg[i..], 42, false)?;
                max_num_31s += 1;
            }
        } else {
            match rule {
                Rule::Letter(c) => match msg.chars().next() {
                    Some(char) if char == c && (msg.len() == 1 || !is_end) => Some(1),
                    _ => None,
                },
                Rule::Rules(rules) => self.match_many(msg, rules, is_end),

                Rule::Either(rules_a, rules_b) => {
                    if rules_a.len() != rules_b.len() {
                        panic!("Variable length rule: {}", rule_id);
                    }

                    self.match_many(msg, rules_a, is_end)
                        .or_else(|| self.match_many(msg, rules_b, is_end))
                }
            }
        }
    }

    fn match_many(&self, msg: &str, rules: RuleIds, is_end: bool) -> Option<usize> {
        if rules.len() > msg.len() {
            return None;
        }

        let mut x = 0;

        for (i, &rule) in rules.iter().enumerate() {
            let is_last = i + 1 >= rules.len();

            x += self.match_length(&msg[x..], rule, is_last && is_end)?;
        }

        Some(x)
    }

    pub fn matches_repeating_31s(&self, msg: &str, mut max_times: usize) -> Option<usize> {
        let mut i = 0;

        while max_times > 0 {
            if let Some(x) = self.match_length(&msg[i..], 31, true) {
                return Some(i + x);
            }

            i += self.match_length(&msg[i..], 31, false)?;

            max_times -= 1;
        }

        None
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

    const EXAMPLE_TWO: &str = "
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1
";

    #[test_case("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa", false)]
    #[test_case("bbabbbbaabaabba", true)]
    #[test_case("babbbbaabbbbbabbbbbbaabaaabaaa", false)]
    #[test_case("aaabbbbbbaaaabaababaabababbabaaabbababababaaa", false)]
    #[test_case("bbbbbbbaaaabbbbaaabbabaaa", false)]
    #[test_case("bbbababbbbaaaaaaaabbababaaababaabab", false)]
    #[test_case("ababaaaaaabaaab", true)]
    #[test_case("ababaaaaabbbaba", true)]
    #[test_case("baabbaaaabbaaaababbaababb", false)]
    #[test_case("abbbbabbbbaaaababbbbbbaaaababb", false)]
    #[test_case("aaaaabbaabaaaaababaa", false)]
    #[test_case("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa", false)]
    #[test_case("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba", false)]
    fn initial_sample_input_part_two(msg: &str, is_valid: bool) {
        let rules = parse_rules(&mut EXAMPLE_TWO.trim().lines());

        assert_eq!(rules.is_valid(msg), is_valid);
    }

    #[test_case("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa", false)]
    #[test_case("bbabbbbaabaabba", true)]
    #[test_case("babbbbaabbbbbabbbbbbaabaaabaaa", true)]
    #[test_case("aaabbbbbbaaaabaababaabababbabaaabbababababaaa", true)]
    #[test_case("bbbbbbbaaaabbbbaaabbabaaa", true)]
    #[test_case("bbbababbbbaaaaaaaabbababaaababaabab", true)]
    #[test_case("ababaaaaaabaaab", true)]
    #[test_case("ababaaaaabbbaba", true)]
    #[test_case("baabbaaaabbaaaababbaababb", true)]
    #[test_case("abbbbabbbbaaaababbbbbbaaaababb", true)]
    #[test_case("aaaaabbaabaaaaababaa", true)]
    #[test_case("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa", true)]
    #[test_case("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba", true)]
    fn updated_sample_input_part_two(msg: &str, is_valid: bool) {
        let mut rules = parse_rules(&mut EXAMPLE_TWO.trim().lines());

        update_rules_8_and_11(&mut rules);

        assert_eq!(rules.is_valid(msg), is_valid);
    }
}
