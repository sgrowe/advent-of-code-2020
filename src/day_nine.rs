use super::pairs::Pairs;
use super::utils::{parse_ints, start_day};
use std::cmp::Ordering;

pub fn main() {
    let input = start_day("nine");

    let ints: Vec<u64> = parse_ints(&input).collect();

    println!("Part one: {}", first_invalid_number(&mut [0; 25], &ints));
    println!("Part two: {}", part_two(&mut [0; 25], &ints));
    println!();
}

fn first_invalid_number(mut prev_nums: &mut [u64], input: &[u64]) -> u64 {
    for (i, &x) in input.iter().enumerate() {
        if i < prev_nums.len() {
            prev_nums[i] = x;
        } else {
            if !Pairs::of(&prev_nums).any(|(a, b)| a != b && a + b == x) {
                return x;
            }

            append_item(&mut prev_nums, x);
        }
    }

    panic!("Solution not found");
}

fn append_item(window: &mut [u64], x: u64) {
    let end = window.len() - 1;

    for i in 0..end {
        window[i] = window[i + 1];
    }

    window[end] = x;
}

fn part_two(mut prev_nums: &mut [u64], input: &[u64]) -> u64 {
    let invalid = first_invalid_number(&mut prev_nums, input);

    for i in 0.. {
        let mut sum = input[i];

        for j in i + 1.. {
            sum += input[j];

            match sum.cmp(&invalid) {
                Ordering::Greater => {
                    break;
                }
                Ordering::Equal => {
                    let range = &input[i..j + 1];

                    let min = range.iter().min().unwrap();
                    let max = range.iter().max().unwrap();

                    return min + max;
                }
                _ => {}
            }
        }
    }

    panic!("Solution not found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

    #[test]
    fn sample_input_part_one() {
        let input: Vec<_> = parse_ints(SAMPLE_INPUT.trim()).collect();

        assert_eq!(first_invalid_number(&mut [0; 5], &input), 127);
    }

    #[test]
    fn long_test_input_part_one() {
        let input = "
20
1
2
3
4
5
6
7
8
9
10
11
12
13
14
15
16
17
18
19
21
22
23
24
25
45
65
";

        let input: Vec<_> = parse_ints(input.trim()).collect();

        assert_eq!(first_invalid_number(&mut [0; 25], &input), 65);
    }

    #[test]
    fn sample_input_part_two() {
        let input: Vec<_> = parse_ints(SAMPLE_INPUT.trim()).collect();

        assert_eq!(part_two(&mut [0; 5], &input), 62);
    }
}
