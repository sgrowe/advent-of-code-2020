use super::utils::read_input_file;

pub fn main() {
    println!("Day three");

    let input = read_input_file("three");

    let map = Map::parse(&input);

    println!("Part one: {}", part_one(&map));
    println!("Part two: {}", part_two(&map));
    println!();
}

fn part_one(map: &Map) -> usize {
    map.toboggan_path(3, 1)
        .filter(|&point| point == Point::Tree)
        .count()
}

fn part_two(map: &Map) -> usize {
    let toboggans = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut result = 1;

    for &(step_x, step_y) in &toboggans {
        let trees_hit = map
            .toboggan_path(step_x, step_y)
            .filter(|&point| point == Point::Tree)
            .count();

        result *= trees_hit;
    }

    result
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Point {
    Empty,
    Tree,
}

impl Point {
    pub fn from_char(c: char) -> Self {
        match c {
            '.' => Point::Empty,
            '#' => Point::Tree,
            _ => panic!(),
        }
    }
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<Point>>,
}

impl Map {
    pub fn parse(text: &str) -> Self {
        let grid = text
            .lines()
            .map(|line| line.chars().map(Point::from_char).collect())
            .collect();

        Map { grid }
    }

    pub fn toboggan_path(&self, step_x: usize, step_y: usize) -> TobogganIterator {
        TobogganIterator {
            map: self,
            x: 0,
            y: 0,
            step_x,
            step_y,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct TobogganIterator<'a> {
    map: &'a Map,
    x: usize,
    y: usize,
    step_x: usize,
    step_y: usize,
}

impl<'a> Iterator for TobogganIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.map.grid.get(self.y)?;

        let x = self.x % line.len();

        self.x += self.step_x;
        self.y += self.step_y;

        Some(line[x])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

    #[test]
    fn sample_input_part_one() {
        let map = Map::parse(TEST_INPUT.trim());

        assert_eq!(part_one(&map), 7);
    }

    #[test]
    fn sample_input_part_two() {
        let map = Map::parse(TEST_INPUT.trim());

        assert_eq!(part_two(&map), 336);
    }
}
