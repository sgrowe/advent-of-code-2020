use super::utils::{parse_ints, start_day};
use std::collections::HashMap;

pub fn main() {
    let input = start_day("ten");

    let ints: Vec<usize> = parse_ints(&input).collect();

    let adapters = Adapters::from_ratings(&ints);

    println!("Part one: {}", part_one(&adapters));
    println!("Part two: {}", part_two(&adapters));
    println!();
}

fn part_one(adapters: &Adapters) -> usize {
    let mut one_jolts_count = 0;
    let mut three_jolts_count = 0;

    let mut prev_rating = 0;

    for rating in adapters.use_all_of_them() {
        match rating - prev_rating {
            1 => {
                one_jolts_count += 1;
            }
            3 => {
                three_jolts_count += 1;
            }
            _ => {}
        }

        prev_rating = rating
    }

    one_jolts_count * three_jolts_count
}

fn part_two(adapters: &Adapters) -> usize {
    let mut cache = HashMap::with_capacity(adapters.count);

    num_valid_arrangements(adapters, 0, &mut cache)
}

fn num_valid_arrangements(
    adapters: &Adapters,
    i: usize,
    mut cache: &mut HashMap<usize, usize>,
) -> usize {
    adapters
        .valid_next_adapters(i)
        .map(|x| {
            if x == adapters.device_rating() {
                return 1;
            }

            cache.get(&x).copied().unwrap_or_else(|| {
                let count = num_valid_arrangements(adapters, x, &mut cache);

                cache.insert(x, count);

                count
            })
        })
        .sum()
}

struct Adapters {
    adapters: Vec<bool>,
    count: usize,
}

impl Adapters {
    pub fn from_ratings(ratings: &[usize]) -> Self {
        let max = *ratings.iter().max().unwrap();

        let mut adapters = vec![false; max + 4];

        for &x in ratings {
            adapters[x] = true;
        }

        adapters[max + 3] = true;

        Adapters {
            adapters,
            count: ratings.len(),
        }
    }

    pub fn use_all_of_them(&self) -> PartOneIterator {
        PartOneIterator {
            adapters: self,
            cur_rating: 0,
        }
    }

    pub fn device_rating(&self) -> usize {
        self.adapters.len() - 1
    }

    pub fn valid_next_adapters<'a>(&'a self, i: usize) -> impl Iterator<Item = usize> + 'a {
        let next_min = i + 1;
        let next_max = i + 3;

        (next_min..=next_max).filter(move |&i| self.adapters.get(i).copied().unwrap_or_default())
    }
}

struct PartOneIterator<'a> {
    adapters: &'a Adapters,
    cur_rating: usize,
}

impl<'a> Iterator for PartOneIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.adapters.valid_next_adapters(self.cur_rating).next()?;

        self.cur_rating = next;

        Some(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_EXAMPLE: &str = "
16
10
15
5
1
11
7
19
6
12
4
";

    const SECOND_EXAMPLE: &str = "
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";

    #[test]
    fn first_sample_input_part_one() {
        let ints: Vec<_> = parse_ints(FIRST_EXAMPLE.trim()).collect();

        assert_eq!(part_one(&Adapters::from_ratings(&ints)), 7 * 5);
    }

    #[test]
    fn second_sample_input_part_one() {
        let ints: Vec<_> = parse_ints(SECOND_EXAMPLE.trim()).collect();

        assert_eq!(part_one(&Adapters::from_ratings(&ints)), 22 * 10);
    }

    #[test]
    fn first_sample_input_part_two() {
        let ints: Vec<_> = parse_ints(FIRST_EXAMPLE.trim()).collect();

        assert_eq!(part_two(&Adapters::from_ratings(&ints)), 8);
    }

    #[test]
    fn second_sample_input_part_two() {
        let ints: Vec<_> = parse_ints(SECOND_EXAMPLE.trim()).collect();

        assert_eq!(part_two(&Adapters::from_ratings(&ints)), 19208);
    }
}
