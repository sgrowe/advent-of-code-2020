use super::utils::start_day;

pub fn main() {
    let input = start_day("eleven");

    println!("Part one: {}", part_one(&input));
    // println!("Part two: {}", part_two(&input));
    println!();
}

fn part_one(input: &str) -> usize {
    let mut seats = SeatLayout::parse(input);

    seats.run();

    seats.total_occupied_seats()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Position {
    Floor,
    Empty,
    Filled,
}

impl Position {
    pub fn parse(c: char) -> Self {
        match c {
            '.' => Position::Floor,
            'L' => Position::Empty,
            '#' => Position::Filled,
            _ => panic!("Unexpected char: {}", c),
        }
    }

    pub fn next(&self, num_occupied_adjacent_seat: usize) -> Self {
        match (self, num_occupied_adjacent_seat) {
            (Position::Empty, 0) => Position::Filled,
            (Position::Empty, _) => Position::Empty,
            (Position::Filled, x) if x >= 4 => Position::Empty,
            (Position::Filled, _) => Position::Filled,
            (Position::Floor, _) => Position::Floor,
        }
    }
}

#[derive(Debug)]
struct SeatLayout {
    seats: Vec<Vec<Position>>,
}

impl SeatLayout {
    pub fn parse(input: &str) -> Self {
        let seats = input
            .lines()
            .map(|l| l.chars().map(Position::parse).collect())
            .collect();

        SeatLayout { seats }
    }

    pub fn run(&mut self) {
        let row = vec![0; self.seats[0].len()];
        let mut num_occupied = vec![row; self.seats.len()];

        loop {
            for i in 0..self.seats.len() {
                for j in 0..self.seats[0].len() {
                    num_occupied[i][j] = self.occupied_adjacent_seats(i, j);
                }
            }

            let mut changed_count: usize = 0;

            for i in 0..self.seats.len() {
                for j in 0..self.seats[0].len() {
                    let old = self.get(i, j).unwrap();
                    let new = old.next(num_occupied[i][j]);

                    if old != new {
                        changed_count += 1;
                        self.seats[i][j] = new;
                    }
                }
            }

            if changed_count == 0 {
                return;
            }
        }
    }

    pub fn total_occupied_seats(&self) -> usize {
        self.seats
            .iter()
            .map(|row| row.iter().filter(|&p| *p == Position::Filled).count())
            .sum()
    }

    fn occupied_adjacent_seats(&self, i: usize, j: usize) -> usize {
        let i_min = if i > 0 { i - 1 } else { i };
        let j_min = if j > 0 { j - 1 } else { j };

        let mut count = 0;

        for x in i_min..=i + 1 {
            for y in j_min..=j + 1 {
                if x == i && y == j {
                    continue;
                }

                if self.get(x, y) == Some(Position::Filled) {
                    count += 1;
                }
            }
        }

        count
    }

    fn get(&self, i: usize, j: usize) -> Option<Position> {
        self.seats.get(i).and_then(|row| row.get(j)).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

    #[test]
    fn sample_input_part_one() {
        assert_eq!(part_one(&EXAMPLE.trim()), 37);
    }
}
