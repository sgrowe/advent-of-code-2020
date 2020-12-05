use super::utils::read_input_file;
use std::collections::HashMap;

pub fn main() {
    println!("Day four");

    let input = read_input_file("four");

    println!("Part one: {}", part_one(&input));
    // println!("Part two: {}", part_two(&input));
    println!();
}

fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Passport::parse)
        .filter(|passport| passport.is_valid())
        .count()
}

struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}

impl<'a> Passport<'a> {
    pub fn parse(text: &'a str) -> Self {
        let fields = text
            .split_ascii_whitespace()
            .map(|s| {
                let (k, v) = s.split_at(s.find(':').unwrap());

                (k, &v[1..])
            })
            .collect();

        Passport { fields }
    }

    pub fn is_valid(&self) -> bool {
        let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        required_fields.iter().all(|k| self.fields.contains_key(k))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

    #[test]
    fn sample_input_part_one() {
        assert_eq!(part_one(TEST_INPUT.trim()), 2);
    }

    // #[test]
    // fn sample_input_part_two() {
    //     assert_eq!(part_two(TEST_INPUT), 1);
    // }
}
