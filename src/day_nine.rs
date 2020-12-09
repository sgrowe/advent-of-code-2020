use super::pairs::Pairs;
use super::utils::{parse_ints, start_day};

pub fn main() {
    let input = start_day("nine");

    println!("Part one: {}", part_one(&mut [0; 25], &input));
    // println!("Part two: {}", part_two(&input));
    println!();
}

fn part_one(mut prev_nums: &mut [u64], input: &str) -> u64 {
    for (i, x) in parse_ints(input).enumerate() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_input_part_one() {
        let input = "
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

        assert_eq!(part_one(&mut [0; 5], input.trim()), 127);
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

        assert_eq!(part_one(&mut [0; 25], input.trim()), 65);
    }
}
