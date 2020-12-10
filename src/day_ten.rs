use super::utils::{parse_ints, start_day};

pub fn main() {
    let input = start_day("ten");

    let ints: Vec<usize> = parse_ints(&input).collect();

    println!("Part one: {}", part_one(&ints));
    println!();
}

fn part_one(input: &[usize]) -> usize {
    let mut one_jolts_count = 0;
    let mut three_jolts_count = 0;

    let adapters = Adapters::from_ratings(input);

    let mut prev_rating = 0;

    for rating in adapters {
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

struct Adapters {
    adapters: Vec<bool>,
    cur_rating: usize,
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
            cur_rating: 0,
        }
    }
}

impl Iterator for Adapters {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let next_min = self.cur_rating + 1;
        let next_max = self.cur_rating + 3;

        for i in next_min..=next_max {
            if *self.adapters.get(i)? {
                self.cur_rating = i;
                return Some(i);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_sample_input_part_one() {
        let input = "
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
"
        .trim();

        let ints: Vec<_> = parse_ints(&input).collect();

        assert_eq!(part_one(&ints), 7 * 5);
    }

    #[test]
    fn second_sample_input_part_one() {
        let input = "
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
"
        .trim();

        let ints: Vec<_> = parse_ints(&input).collect();

        assert_eq!(part_one(&ints), 22 * 10);
    }
}
