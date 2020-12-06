use super::utils::{parse_ints, read_input_file};

pub fn main() {
    println!("Day one");

    let input = read_input_file("one");

    let numbers: Vec<u64> = parse_ints(&input).collect();

    println!("Part one: {}", part_one(&numbers));
    println!("Part two: {}", part_two(&numbers));
    println!();
}

fn part_one(input: &[u64]) -> u64 {
    for (i, x) in input.iter().enumerate() {
        for y in &input[i + 1..] {
            if x + y == 2020 {
                return x * y;
            }
        }
    }

    panic!()
}

fn part_two(input: &[u64]) -> u64 {
    for (i, x) in input.iter().enumerate() {
        for (j, y) in (&input[i + 1..]).iter().enumerate() {
            for z in &input[j + 1..] {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }

    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
1721
979
366
299
675
1456
";

    #[test]
    fn sample_input_part_one() {
        let ints: Vec<u64> = parse_ints(TEST_INPUT.trim()).collect();

        assert_eq!(part_one(&ints), 514579);
    }

    #[test]
    fn sample_input_part_two() {
        let ints: Vec<u64> = parse_ints(TEST_INPUT.trim()).collect();

        assert_eq!(part_two(&ints), 241861950);
    }
}
