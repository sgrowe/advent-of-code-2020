use super::utils::start_day;

pub fn main() {
    let input = start_day("thirteen");

    let buses = BusSchedule::parse(&input);

    println!("Part one: {}", part_one(&buses));
    // println!("Part two: {}", part_two(&buses));
    println!();
}

fn part_one(buses: &BusSchedule) -> usize {
    let (id, next_dept) = buses.next_dept();

    let wait_time = next_dept - buses.earliest_time;

    id * wait_time
}

struct BusSchedule {
    earliest_time: usize,
    times: Vec<usize>,
}

impl BusSchedule {
    pub fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let earliest_time = lines.next().unwrap().parse().unwrap();
        let times = lines
            .next()
            .unwrap()
            .split(',')
            .filter_map(|t| match t {
                "x" => None,
                _ => t.parse().ok(),
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
            .map(|&id| {
                let rem = self.earliest_time % id;

                (id, (self.earliest_time + id) - rem)
            })
            .min_by_key(|&(_, next_dept)| next_dept)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
939
7,13,x,x,59,x,31,19
";

    #[test]
    fn sample_input_part_one() {
        let buses = BusSchedule::parse(&EXAMPLE.trim());

        assert_eq!(part_one(&buses), 295);
    }
}
