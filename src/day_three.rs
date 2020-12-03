use super::utils::read_input_file;

pub fn main() {
    println!("Day three");

    let input = read_input_file("three");

    println!("Part one: {}", part_one(&input));
    // println!("Part two: {}", part_two(&input));
    println!();
}

fn part_one(input: &str) -> usize {
    let map = Map::parse(input);

    map.toboggan_path()
        .filter(|&point| point == Point::Tree)
        .count()
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

    pub fn toboggan_path(&self) -> TobogganIterator {
        TobogganIterator {
            map: self,
            x: 0,
            y: 0,
        }
    }
}

struct TobogganIterator<'a> {
    map: &'a Map,
    x: usize,
    y: usize,
}

impl<'a> Iterator for TobogganIterator<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let line = self.map.grid.get(self.y)?;

        let x = self.x % line.len();

        self.x += 3;
        self.y += 1;

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
        assert_eq!(part_one(TEST_INPUT.trim()), 7);
    }
}
