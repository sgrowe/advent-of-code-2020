use super::utils::read_input_file;

pub fn main() {
    println!("Day five");

    let input = read_input_file("five");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
    println!();
}

fn part_one(input: &str) -> u16 {
    input
        .lines()
        .map(|l| {
            let ticket = Ticket::parse(l);

            ticket.seat_id()
        })
        .max()
        .unwrap_or_default()
}

fn part_two(input: &str) -> usize {
    let mut seats_taken = [false; 1024];

    let seat_ids = input.lines().map(|l| {
        let ticket = Ticket::parse(l);

        ticket.seat_id() as usize
    });

    for id in seat_ids {
        seats_taken[id] = true;
    }

    for id in 1..1023 {
        if seats_taken[id - 1] && seats_taken[id + 1] && !seats_taken[id] {
            return id;
        }
    }

    panic!("Failed to find seat")
}

#[derive(Debug, Copy, Clone)]
enum RowInstr {
    Front,
    Back,
}

#[derive(Debug, Copy, Clone)]
enum ColInstr {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Ticket {
    row_steps: [RowInstr; 7],
    col_steps: [ColInstr; 3],
}

impl Ticket {
    pub fn parse(text: &str) -> Self {
        let mut chars = text.chars();

        let mut row_steps = [RowInstr::Front; 7];
        let mut col_steps = [ColInstr::Left; 3];

        for instr in row_steps.iter_mut() {
            *instr = match chars.next().unwrap() {
                'F' => RowInstr::Front,
                'B' => RowInstr::Back,
                c => panic!("Unexpected char: {}", c),
            };
        }

        for instr in col_steps.iter_mut() {
            *instr = match chars.next().unwrap() {
                'L' => ColInstr::Left,
                'R' => ColInstr::Right,
                c => panic!("Unexpected char: {}", c),
            };
        }

        Ticket {
            row_steps,
            col_steps,
        }
    }

    fn position(&self) -> (u16, u16) {
        let mut row_hi = 127;
        let mut row_lo = 0;

        for step in &self.row_steps {
            let rows_change = ((row_hi - row_lo) + 1) / 2;

            match step {
                RowInstr::Front => {
                    row_hi -= rows_change;
                }
                RowInstr::Back => {
                    row_lo += rows_change;
                }
            }
        }

        let mut col_hi = 7;
        let mut col_lo = 0;

        for step in &self.col_steps {
            let cols_change = ((col_hi - col_lo) + 1) / 2;

            match step {
                ColInstr::Left => {
                    col_hi -= cols_change;
                }
                ColInstr::Right => {
                    col_lo += cols_change;
                }
            }
        }

        (row_hi, col_hi)
    }

    pub fn seat_id(&self) -> u16 {
        let (row, col) = self.position();

        (row * 8) + col
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seat_position_correct() {
        assert_eq!(Ticket::parse("BFFFBBFRRR").position(), (70, 7));
        assert_eq!(Ticket::parse("FFFBBBFRRR").position(), (14, 7));
        assert_eq!(Ticket::parse("BBFFBBFRLL").position(), (102, 4));
    }

    #[test]
    fn seat_id_correct() {
        assert_eq!(Ticket::parse("BFFFBBFRRR").seat_id(), 567);
        assert_eq!(Ticket::parse("FFFBBBFRRR").seat_id(), 119);
        assert_eq!(Ticket::parse("BBFFBBFRLL").seat_id(), 820);
    }
}
