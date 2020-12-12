use super::utils::start_day;

pub fn main() {
    let input = start_day("twelve");

    let instr = Intructions::parse(&input);

    println!("Part one: {}", part_one(&instr));
    println!("Part two: {}", part_two(&instr));
    println!();
}

fn part_one(instr: &Intructions) -> i64 {
    let ship = instr.run(Ship::new());

    ship.manhattan_distance()
}

fn part_two(instr: &Intructions) -> i64 {
    let ship = instr.run(ShipWithWaypoint::new());

    ship.manhattan_distance()
}

#[derive(Debug, Copy, Clone)]
enum Action {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}

impl Action {
    pub fn parse(c: char) -> Self {
        match c {
            'N' => Action::N,
            'S' => Action::S,
            'E' => Action::E,
            'W' => Action::W,
            'L' => Action::L,
            'R' => Action::R,
            'F' => Action::F,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Copy, Clone)]
struct Ship {
    dir: Dir,
    x: i64,
    y: i64,
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            dir: Dir::East,
            x: 0,
            y: 0,
        }
    }

    fn rotate_left(&mut self) {
        self.dir = match self.dir {
            Dir::North => Dir::West,
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
        };
    }

    fn move_dir(&mut self, dir: Dir, amount: i64) {
        match dir {
            Dir::North => self.y += amount,
            Dir::East => self.x += amount,
            Dir::South => self.y -= amount,
            Dir::West => self.x -= amount,
        }
    }

    pub fn manhattan_distance(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

trait Moveable {
    fn update(&mut self, action: Action, amount: i64);
}

impl Moveable for Ship {
    fn update(&mut self, action: Action, amount: i64) {
        match action {
            Action::F => self.move_dir(self.dir, amount),
            Action::L => {
                for _ in 0..amount / 90 {
                    self.rotate_left()
                }
            }
            Action::R => {
                for _ in 0..(360 - amount) / 90 {
                    self.rotate_left()
                }
            }
            Action::N => self.move_dir(Dir::North, amount),
            Action::S => self.move_dir(Dir::South, amount),
            Action::E => self.move_dir(Dir::East, amount),
            Action::W => self.move_dir(Dir::West, amount),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct ShipWithWaypoint {
    ship: Ship,
    waypoint: (i64, i64),
}

impl ShipWithWaypoint {
    pub fn new() -> Self {
        ShipWithWaypoint {
            ship: Ship::new(),
            waypoint: (10, 1),
        }
    }

    pub fn manhattan_distance(&self) -> i64 {
        self.ship.manhattan_distance()
    }

    fn move_waypoint(&mut self, dir: Dir, amount: i64) {
        match dir {
            Dir::North => self.waypoint.1 += amount,
            Dir::East => self.waypoint.0 += amount,
            Dir::South => self.waypoint.1 -= amount,
            Dir::West => self.waypoint.0 -= amount,
        }
    }

    fn rotate_waypoint_left(&mut self) {
        let (x, y) = self.waypoint;

        self.waypoint = (-y, x);
    }
}

impl Moveable for ShipWithWaypoint {
    fn update(&mut self, action: Action, amount: i64) {
        match action {
            Action::F => {
                let (x, y) = self.waypoint;

                self.ship.x += x * amount;
                self.ship.y += y * amount;
            }
            Action::L => {
                for _ in 0..amount / 90 {
                    self.rotate_waypoint_left()
                }
            }
            Action::R => {
                for _ in 0..(360 - amount) / 90 {
                    self.rotate_waypoint_left()
                }
            }
            Action::N => self.move_waypoint(Dir::North, amount),
            Action::S => self.move_waypoint(Dir::South, amount),
            Action::E => self.move_waypoint(Dir::East, amount),
            Action::W => self.move_waypoint(Dir::West, amount),
        }
    }
}

struct Intructions {
    instr: Vec<(Action, i64)>,
}

impl Intructions {
    pub fn parse(input: &str) -> Self {
        let instr = input
            .lines()
            .map(|l| {
                let action = Action::parse(l.chars().next().unwrap());
                let amount = (&l[1..]).parse().unwrap();

                (action, amount)
            })
            .collect();

        Intructions { instr }
    }

    pub fn run<S: Moveable>(&self, mut ship: S) -> S {
        for &(action, amount) in &self.instr {
            ship.update(action, amount);
        }

        ship
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "
F10
N3
F7
R90
F11
";

    #[test]
    fn sample_input_part_one() {
        let instr = Intructions::parse(&EXAMPLE.trim());

        assert_eq!(part_one(&instr), 25);
    }

    #[test]
    fn sample_input_part_two() {
        let instr = Intructions::parse(&EXAMPLE.trim());

        assert_eq!(part_two(&instr), 286);
    }
}
