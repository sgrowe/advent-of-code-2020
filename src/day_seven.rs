use super::utils::{capture_to_str, start_day};
use regex::Regex;
use std::collections::HashMap;

pub fn main() {
    let input = start_day("seven");

    let bags = Bags::parse(&input);

    println!("Part one: {}", bags.num_gold_bag_containers());
    // println!("Part two: {}", part_two(&passports));
    println!();
}

struct Bags<'a> {
    bags: HashMap<&'a str, Vec<(usize, &'a str)>>,
}

impl<'a> Bags<'a> {
    pub fn parse(input: &'a str) -> Self {
        let bags = input
            .lines()
            .map(|l| {
                let contain = " bags contain ";
                let mid_point = l.find(contain).unwrap();

                let name = &l[..mid_point];

                let contains = mid_point + contain.len();

                let inner_bags = match &l[contains..] {
                    "no other bags." => vec![],
                    rest => parse_inner_bags(rest),
                };

                (name, inner_bags)
            })
            .collect();

        Bags { bags }
    }

    pub fn num_gold_bag_containers(&self) -> usize {
        let mut gold_bag_containers = HashMap::with_capacity(self.bags.len());

        let mut result = 0;

        for bag in self.bags.keys() {
            if self.gold(bag, &mut gold_bag_containers) > 0 {
                result += 1;
            }
        }

        result
    }

    fn gold(&self, bag: &'a str, mut cache: &mut HashMap<&'a str, usize>) -> usize {
        cache.get(bag).copied().unwrap_or_else(|| {
            let gold_bags = self.bags[bag]
                .iter()
                .map(|&(count, inner_bag)| {
                    if inner_bag == "shiny gold" {
                        return count;
                    }

                    let gold_bags = count * self.gold(inner_bag, &mut cache);

                    cache.insert(inner_bag, gold_bags);

                    gold_bags
                })
                .sum();

            gold_bags
        })
    }
}

fn parse_inner_bags(s: &str) -> Vec<(usize, &str)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
    }

    RE.captures_iter(s)
        .map(|cap| {
            let num = capture_to_str(&cap, 1).parse().unwrap();
            let bag_type = capture_to_str(&cap, 2);

            (num, bag_type)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
";

    #[test]
    fn sample_input_part_one() {
        let bags = Bags::parse(TEST_INPUT.trim());

        assert_eq!(bags.num_gold_bag_containers(), 4);
    }
}
