use super::utils::start_day;

pub fn main() {
    let input = start_day("eleven");

    let seats = SeatLayout::parse(&input);

    println!("Part one: {}", part_one(seats.clone()));
    println!("Part two: {}", part_two(seats));
    println!();
}

fn part_one(mut seats: SeatLayout) -> usize {
    seats.run(false);

    seats.total_occupied_seats()
}

fn part_two(mut seats: SeatLayout) -> usize {
    seats.run(true);

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

    pub fn next(&self, num_occupied_adjacent_seat: usize, tolerance: usize) -> Self {
        match (self, num_occupied_adjacent_seat) {
            (Position::Empty, 0) => Position::Filled,
            (Position::Empty, _) => Position::Empty,
            (Position::Filled, x) if x >= tolerance => Position::Empty,
            (Position::Filled, _) => Position::Filled,
            (Position::Floor, _) => Position::Floor,
        }
    }
}

#[derive(Debug, Clone)]
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

    pub fn run(&mut self, v2: bool) {
        let row = vec![0; self.seats[0].len()];
        let mut num_occupied = vec![row; self.seats.len()];

        loop {
            for i in 0..self.seats.len() {
                for j in 0..self.seats[0].len() {
                    num_occupied[i][j] = if v2 {
                        self.occupied_visible_seats(i, j)
                    } else {
                        self.occupied_adjacent_seats(i, j)
                    };
                }
            }

            let mut changed_count: usize = 0;

            for i in 0..self.seats.len() {
                for j in 0..self.seats[0].len() {
                    let tolerance = if v2 { 5 } else { 4 };

                    let old = self.get(i, j).unwrap();
                    let new = old.next(num_occupied[i][j], tolerance);

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
        let i_min = i.saturating_sub(1);
        let j_min = j.saturating_sub(1);

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

    fn occupied_visible_seats(&self, i: usize, j: usize) -> usize {
        let changes = [Change::Incr, Change::Same, Change::Decr];

        let mut count = 0;

        for &a in &changes {
            for &b in &changes {
                if a == Change::Same && b == Change::Same {
                    continue;
                }

                if let Some(Position::Filled) = self.next_visible_seat((i, j), (a, b)) {
                    count += 1;
                }
            }
        }

        count
    }

    fn next_visible_seat(
        &self,
        (mut x, mut y): (usize, usize),
        (x_dir, y_dir): (Change, Change),
    ) -> Option<Position> {
        loop {
            x = x_dir.step(x)?;
            y = y_dir.step(y)?;

            let pos = self.get(x, y)?;

            if pos != Position::Floor {
                return Some(pos);
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Change {
    Incr,
    Same,
    Decr,
}

impl Change {
    pub fn step(&self, x: usize) -> Option<usize> {
        match self {
            Change::Incr => x.checked_add(1),
            Change::Same => Some(x),
            Change::Decr => x.checked_sub(1),
        }
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
        let seats = SeatLayout::parse(&EXAMPLE.trim());

        assert_eq!(part_one(seats), 37);
    }

    #[test]
    fn sample_input_part_two() {
        let seats = SeatLayout::parse(&EXAMPLE.trim());

        assert_eq!(part_two(seats), 26);
    }
}
