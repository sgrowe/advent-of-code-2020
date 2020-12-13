use super::utils::start_day;

pub fn main() {
    let input = start_day("eleven");

    let seats = SeatLayout::parse(&input);

    println!("Part one: {}", part_one(seats.clone()));
    println!("Part two: {}", part_two(seats));
    println!();
}

fn part_one(mut seats: SeatLayout) -> usize {
    seats.run(SeatLayout::occupied_adjacent_seats, 4);

    seats.total_occupied_seats()
}

fn part_two(mut seats: SeatLayout) -> usize {
    seats.run(SeatLayout::occupied_visible_seats, 5);

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

    pub fn update(&self, num_occupied_adjacent_seat: usize, tolerance: usize) -> Self {
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

    pub fn run<F>(&mut self, num_occupied_seats: F, tolerance: usize)
    where
        F: Fn(&SeatLayout, usize, usize) -> usize,
    {
        let locations = self.seat_locations();

        let row_len = self.seats[0].len();
        let mut num_occupied = vec![0; self.seats.len() * row_len];

        loop {
            for &(i, j) in &locations {
                num_occupied[(i * row_len) + j] = num_occupied_seats(&self, i, j);
            }

            let mut has_changed = false;

            for &(i, j) in &locations {
                let old = self.seats[i][j];
                let new = old.update(num_occupied[(i * row_len) + j], tolerance);

                if old != new {
                    has_changed = true;
                    self.seats[i][j] = new;
                }
            }

            if !has_changed {
                return;
            }
        }
    }

    pub fn total_occupied_seats(&self) -> usize {
        self.seats
            .iter()
            .flatten()
            .filter(|&p| *p == Position::Filled)
            .count()
    }

    fn seat_locations(&self) -> Vec<(usize, usize)> {
        let mut seat_locations = Vec::new();

        for i in 0..self.seats.len() {
            for j in 0..self.seats[0].len() {
                match self.seats[i][j] {
                    Position::Floor => {}
                    _ => seat_locations.push((i, j)),
                }
            }
        }

        seat_locations
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
