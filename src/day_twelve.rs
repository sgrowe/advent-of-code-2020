use super::utils::start_day;

pub fn main() {
    let input = start_day("twelve");

    let instr = Intructions::parse(&input);

    println!("Part one: {}", part_one(&instr));
    // println!("Part two: {}", part_two(&instr));
    println!();
}

fn part_one(instr: &Intructions) -> i64 {
    let ship = instr.run();

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

    pub fn update(&mut self, action: Action, amount: i64) {
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

    pub fn run(&self) -> Ship {
        let mut ship = Ship::new();

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
}
