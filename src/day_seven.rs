use super::utils::start_day;
use std::collections::HashMap;

pub fn main() {
    let input = start_day("seven");

    let bags = Bags::parse(&input);

    println!("Part one: {}", bags.num_gold_bag_containers());
    println!("Part two: {}", bags.bags_within_bag("shiny gold"));
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
        let mut gold_bags_per_bag = HashMap::with_capacity(self.bags.len());

        self.bags
            .keys()
            .map(|bag| self.gold(bag, &mut gold_bags_per_bag))
            .filter(|&x| x > 0)
            .count()
    }

    fn gold(&self, bag: &'a str, mut gold_bags_per_bag: &mut HashMap<&'a str, usize>) -> usize {
        gold_bags_per_bag.get(bag).copied().unwrap_or_else(|| {
            let gold_bags = self.bags[bag]
                .iter()
                .map(|&(count, inner_bag)| {
                    if inner_bag == "shiny gold" {
                        return count;
                    }

                    let gold_bags = count * self.gold(inner_bag, &mut gold_bags_per_bag);

                    gold_bags_per_bag.insert(inner_bag, gold_bags);

                    gold_bags
                })
                .sum();

            gold_bags
        })
    }

    pub fn bags_within_bag(&self, bag: &str) -> usize {
        let mut bags_per_bag: HashMap<&str, usize> = HashMap::with_capacity(self.bags.len());

        self.num_bags_with_bag(bag, &mut bags_per_bag)
    }

    fn num_bags_with_bag(&self, bag: &str, mut bags_per_bag: &mut HashMap<&str, usize>) -> usize {
        bags_per_bag.get(bag).copied().unwrap_or_else(|| {
            self.bags[bag]
                .iter()
                .map(|(count, inner_bag)| {
                    count * (1 + self.num_bags_with_bag(inner_bag, &mut bags_per_bag))
                })
                .sum()
        })
    }
}

fn parse_inner_bags(list: &str) -> Vec<(usize, &str)> {
    list.split(',')
        .map(|list| {
            let list = list.trim_start();

            let mut spaces = list.match_indices(' ');

            let num_end = spaces.next().unwrap().0;

            let num = list[..num_end].parse().unwrap();

            spaces.next();
            let bag_end = spaces.next().unwrap().0;

            let bag_name = &list[num_end + 1..bag_end];

            (num, bag_name)
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

    #[test]
    fn sample_input_part_two() {
        let bags = Bags::parse(TEST_INPUT.trim());

        assert_eq!(bags.bags_within_bag("shiny gold"), 32);
    }

    #[test]
    fn second_sample_input_part_two() {
        let input = "
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"
        .trim();

        let bags = Bags::parse(input);

        assert_eq!(bags.bags_within_bag("shiny gold"), 126);
    }
}
