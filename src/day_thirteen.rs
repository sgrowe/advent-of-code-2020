use super::prime_factors::PrimeFactors;
use super::utils::start_day;
use std::collections::HashMap;

pub fn main() {
    let input = start_day("thirteen");

    let buses = BusSchedule::parse(&input);

    println!("Part one: {}", part_one(&buses));
    println!("Part two: {}", part_two(&buses));
    println!();
}

fn part_one(buses: &BusSchedule) -> usize {
    let (id, next_dept) = buses.next_dept();

    let wait_time = next_dept - buses.earliest_time;

    id * wait_time
}

fn part_two(buses: &BusSchedule) -> usize {
    let buses = buses.sorted_by_desc_id();

    let bus_ids: Vec<_> = buses.iter().map(|bus| bus.id).collect();

    let first_bus = buses[0];

    let mut time = first_bus.id - first_bus.index;
    let mut step = first_bus.id;

    for (i, next_bus) in buses.iter().enumerate().skip(1) {
        while (time + next_bus.index) % next_bus.id != 0 {
            time += step;
        }

        let ids_so_far = &bus_ids[..i + 1];

        step = lowest_common_multiple(&ids_so_far);
    }

    time
}

fn lowest_common_multiple(ints: &[usize]) -> usize {
    let mut all_primes: HashMap<usize, usize> = HashMap::new();

    for &x in ints {
        let mut primes = HashMap::new();

        for p in PrimeFactors::of(x) {
            let count = primes.get(&p).copied().unwrap_or_default();

            primes.insert(p, count + 1);
        }

        for (p, count) in primes {
            let prev_count = all_primes.get(&p).copied().unwrap_or_default();

            all_primes.insert(p, prev_count.max(count));
        }
    }

    all_primes.iter().map(|(p, count)| p * count).product()
}

#[derive(Debug, Copy, Clone)]
struct Bus {
    id: usize,
    index: usize,
}

struct BusSchedule {
    earliest_time: usize,
    times: Vec<Bus>,
}

impl BusSchedule {
    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let earliest_time = lines.next().unwrap().parse().unwrap();
        let times = lines
            .next()
            .unwrap()
            .split(',')
            .enumerate()
            .filter_map(|(index, t)| match t {
                "x" => None,
                _ => {
                    let id = t.parse().ok()?;

                    Some(Bus { id, index })
                }
            })
            .collect();

        BusSchedule {
            earliest_time,
            times,
        }
    }

    pub fn next_dept(&self) -> (usize, usize) {
        self.times
            .iter()
            .map(|&Bus { id, index: _ }| {
                let rem = self.earliest_time % id;

                (id, (self.earliest_time + id) - rem)
            })
            .min_by_key(|&(_, next_dept)| next_dept)
            .unwrap()
    }

    pub fn sorted_by_desc_id(&self) -> Vec<Bus> {
        let mut sorted_buses = self.times.clone();

        sorted_buses.sort_by_key(|bus| bus.id);
        sorted_buses.reverse();

        sorted_buses
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const EXAMPLE: &str = "
939
7,13,x,x,59,x,31,19
";

    #[test]
    fn sample_input_part_one() {
        let buses = BusSchedule::parse(&EXAMPLE.trim());

        assert_eq!(part_one(&buses), 295);
    }

    #[test]
    fn sample_input_part_two() {
        let buses = BusSchedule::parse(&EXAMPLE.trim());

        assert_eq!(part_two(&buses), 1068781);
    }

    #[test_case("17,x,13,19", 3417)]
    #[test_case("67,7,59,61", 754018)]
    #[test_case("67,x,7,59,61", 779210)]
    #[test_case("67,7,x,59,61", 1261476)]
    #[test_case("1789,37,47,1889", 1202161486)]
    fn part_two_examples(buses: &str, expected: usize) {
        assert_eq!(run_part_two_on_schedule(buses), expected);

        fn run_part_two_on_schedule(s: &str) -> usize {
            part_two(&BusSchedule::parse(&format!("0\n{}", s)))
        }
    }
}
