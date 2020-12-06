use super::utils::read_input_file;

pub fn main() {
    println!("Day six");

    let input = read_input_file("six");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
    println!();
}

fn part_one(input: &str) -> usize {
    let groups = input.split("\n\n");

    let mut result = 0;

    for group in groups {
        let mut responses = [false; 26];

        group.lines().flat_map(|l| l.chars()).for_each(|c| {
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
        let mut responses: [u16; 26] = [0; 26];
        let mut num_responders = 0;

        for line in group.lines() {
            num_responders += 1;

            for c in line.chars() {
                let i = (c as usize) - 97;

                responses[i] += 1;
            }
        }

        result += responses.iter().filter(|&x| *x == num_responders).count();
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
