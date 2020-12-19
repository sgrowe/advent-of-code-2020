use super::utils::start_day;
use std::iter::Peekable;
use std::str::CharIndices;

pub fn main() {
    let input = start_day("eighteen");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
    println!();
}

fn part_one(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let mut tokens = Tokeniser::of(l);

            eval_expr(&mut tokens)
        })
        .sum()
}

fn part_two(input: &str) -> u64 {
    input
        .lines()
        .map(|l| {
            let mut tokens = Tokeniser::of(l);

            eval_expr_v2(&mut tokens)
        })
        .sum()
}

fn eval_expr(mut tokens: &mut Tokeniser) -> u64 {
    let mut value = get_number(&mut tokens);

    while let Some(token) = tokens.next() {
        match token {
            Token::Add => {
                value += get_number(&mut tokens);
            }
            Token::Multiply => {
                value *= get_number(&mut tokens);
            }
            Token::CloseParen => return value,
            _ => panic!(),
        }
    }

    value
}

fn get_number(mut tokens: &mut Tokeniser) -> u64 {
    match tokens.next().unwrap() {
        Token::Number(a) => a,
        Token::OpenParen => eval_expr(&mut tokens),
        _ => panic!(),
    }
}

fn eval_expr_v2(mut tokens: &mut Tokeniser) -> u64 {
    let mut value: u64 = get_number_v2(&mut tokens);

    let mut multipliers = Vec::new();

    while let Some(token) = tokens.next() {
        match token {
            Token::Add => {
                value += get_number_v2(&mut tokens);
            }
            Token::Multiply => {
                multipliers.push(value);
                value = get_number_v2(&mut tokens);
            }
            Token::CloseParen => {
                let prod: u64 = multipliers.iter().product();

                return value * prod;
            }
            _ => panic!(),
        }
    }

    let prod: u64 = multipliers.iter().product();
    value * prod
}

fn get_number_v2(mut tokens: &mut Tokeniser) -> u64 {
    match tokens.next().unwrap() {
        Token::Number(a) => a,
        Token::OpenParen => eval_expr_v2(&mut tokens),
        _ => panic!(),
    }
}

#[derive(Debug, Copy, Clone)]
enum Token {
    OpenParen,
    CloseParen,
    Number(u64),
    Add,
    Multiply,
}

struct Tokeniser<'a> {
    text: &'a str,
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Tokeniser<'a> {
    pub fn of(line: &'a str) -> Self {
        Tokeniser {
            text: line,
            chars: line.char_indices().peekable(),
        }
    }
}

impl<'a> Iterator for Tokeniser<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let (i, char) = self.chars.next()?;

        let next = match char {
            ' ' => self.next()?,
            '(' => Token::OpenParen,
            ')' => Token::CloseParen,
            '+' => Token::Add,
            '*' => Token::Multiply,
            c if c.is_ascii_digit() => {
                let start = i;
                let mut end = i + 1;

                while let Some(&(i, c)) = self.chars.peek() {
                    if c.is_ascii_digit() {
                        self.chars.next()?;
                        end = i;
                    } else {
                        break;
                    }
                }

                let num = (&self.text[start..end]).parse().unwrap();

                Token::Number(num)
            }
            c => panic!("Unexpected char {}", c),
        };

        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1 + 2 * 3 + 4 * 5 + 6", 71)]
    #[test_case("1 + (2 * 3) + (4 * (5 + 6))", 51)]
    #[test_case("2 * 3 + (4 * 5)", 26)]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437)]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240)]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632)]
    fn sample_input_part_one(expr: &str, expected: u64) {
        assert_eq!(part_one(&expr), expected);
    }

    #[test_case("1 + 2 * 3 + 4 * 5 + 6", 231)]
    #[test_case("1 + (2 * 3) + (4 * (5 + 6))", 51)]
    #[test_case("2 * 3 + (4 * 5)", 46)]
    #[test_case("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445)]
    #[test_case("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060)]
    #[test_case("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340)]
    fn sample_input_part_two(expr: &str, expected: u64) {
        assert_eq!(part_two(&expr), expected);
    }
}
