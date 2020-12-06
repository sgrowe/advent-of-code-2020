use super::utils::read_input_file;
use regex::Regex;
use std::collections::HashMap;

pub fn main() {
    println!("Day four");

    let input = read_input_file("four");

    let passports = parse_passports(&input);

    println!("Part one: {}", part_one(&passports));
    println!("Part two: {}", part_two(&passports));
    println!();
}

fn parse_passports(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(Passport::parse).collect()
}

fn part_one(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.is_valid())
        .count()
}

fn part_two(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|passport| passport.is_valid_v2())
        .count()
}

struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}

impl<'a> Passport<'a> {
    pub fn parse(text: &'a str) -> Self {
        let mut fields = HashMap::with_capacity(8);

        text.split_ascii_whitespace().for_each(|s| {
            let (key, value) = split_key_value(s);

            fields.insert(key, value);
        });

        Passport { fields }
    }

    pub fn is_valid(&self) -> bool {
        let required_fields = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        required_fields.iter().all(|k| self.fields.contains_key(k))
    }

    pub fn is_valid_v2(&self) -> bool {
        self.valid_year_field("byr", 1920, 2002)
            && self.valid_year_field("iyr", 2010, 2020)
            && self.valid_year_field("eyr", 2020, 2030)
            && self.height_valid()
            && self.hcl_valid()
            && self.ecl_valid()
            && self.pid_valid()
    }

    fn valid_year_field(&self, name: &str, min: u32, max: u32) -> bool {
        self.fields
            .get(name)
            .and_then(|s| s.parse::<u32>().ok())
            .map(|y| y >= min && y <= max)
            .unwrap_or(false)
    }

    fn height_valid(&self) -> bool {
        lazy_static! {
            static ref HGT_RE: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        }

        self.fields
            .get("hgt")
            .and_then(|s| {
                let captures = HGT_RE.captures(s)?;

                let num: u32 = captures.get(1)?.as_str().parse().ok()?;
                let unit = captures.get(2)?.as_str();

                let valid = match unit {
                    "cm" => num >= 150 && num <= 193,
                    "in" => num >= 59 && num <= 76,
                    _ => false,
                };

                Some(valid)
            })
            .unwrap_or(false)
    }

    fn hcl_valid(&self) -> bool {
        lazy_static! {
            static ref HCL_RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }

        self.fields
            .get("hcl")
            .map(|s| HCL_RE.is_match(s))
            .unwrap_or(false)
    }

    fn ecl_valid(&self) -> bool {
        self.fields
            .get("ecl")
            .map(|&s| matches!(s, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"))
            .unwrap_or(false)
    }

    fn pid_valid(&self) -> bool {
        self.fields
            .get("pid")
            .map(|&s| s.len() == 9 && s.chars().all(|c| c.is_ascii_digit()))
            .unwrap_or(false)
    }
}

fn split_key_value(pair: &str) -> (&str, &str) {
    let (k, v) = pair.split_at(pair.find(':').unwrap());

    (k, &v[1..])
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
        assert_eq!(part_one(&parse_passports(TEST_INPUT.trim())), 2);
    }

    #[test]
    fn part_two_invalid_passports() {
        let invalid = "
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"
        .trim();

        assert_eq!(part_two(&parse_passports(invalid)), 0);
    }

    #[test]
    fn part_two_valid_passports() {
        let valid = "
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
"
        .trim();

        assert_eq!(part_two(&parse_passports(valid)), 4);
    }
}
