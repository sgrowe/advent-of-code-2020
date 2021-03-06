use super::utils::start_day;

pub fn main() {
    let input = start_day("six");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
    println!();
}

fn part_one(input: &str) -> usize {
    let groups = input.split("\n\n");

    let mut result = 0;

    for group in groups {
        let mut responses = [false; 26];

        group
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .for_each(|c| {
                let i = (c as usize) - 97;

                responses[i] = true;
            });

        result += responses.iter().filter(|&x| *x).count();
    }

    result
}

fn part_two(input: &str) -> usize {
    let groups = input.split("\n\n");

    let mut result = 0;

    for group in groups {
        let mut responses: [u8; 26] = [0; 26];
        let mut num_responders = 1;

        for c in group.chars() {
            match c {
                '\n' => {
                    num_responders += 1;
                }
                _ => {
                    let i = (c as usize) - 97;

                    responses[i] += 1;
                }
            }
        }

        result += bytecount::naive_count_32(&responses, num_responders);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn sample_input_part_one() {
        assert_eq!(part_one(TEST_INPUT.trim()), 11);
    }

    #[test]
    fn sample_input_part_two() {
        assert_eq!(part_two(TEST_INPUT.trim()), 6);
    }
}
