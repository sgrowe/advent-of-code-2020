use super::utils::{capture_to_str, start_day};

use regex::Regex;

pub fn main() {
    let input = start_day("two");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
    println!();
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(PasswordEntry::parse)
        .filter(|password| password.is_valid_v1())
        .count()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(PasswordEntry::parse)
        .filter(|password| password.is_valid_v2())
        .count()
}

#[derive(Debug, Copy, Clone)]
struct PasswordEntry<'a> {
    min_occurs: usize,
    max_occurs: usize,
    letter: char,
    password: &'a str,
}

impl<'a> PasswordEntry<'a> {
    pub fn parse(line: &'a str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();
        }

        let captures = RE.captures(line).unwrap();

        let min_occurs = capture_to_str(&captures, 1).parse().unwrap();
        let max_occurs = capture_to_str(&captures, 2).parse().unwrap();
        let letter = capture_to_str(&captures, 3).chars().next().unwrap();
        let password = capture_to_str(&captures, 4);

        PasswordEntry {
            min_occurs,
            max_occurs,
            letter,
            password,
        }
    }

    pub fn is_valid_v1(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.letter).count();

        count >= self.min_occurs && count <= self.max_occurs
    }

    pub fn is_valid_v2(&self) -> bool {
        let mut chars = self.password.chars();

        let a = chars.nth(self.min_occurs - 1).unwrap();
        let b = chars.nth(self.max_occurs - (self.min_occurs + 1)).unwrap();

        if a == self.letter {
            b != self.letter
        } else {
            b == self.letter
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn sample_input_part_one() {
        assert_eq!(part_one(TEST_INPUT), 2);
    }

    #[test]
    fn sample_input_part_two() {
        assert_eq!(part_two(TEST_INPUT), 1);
    }
}
