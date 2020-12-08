use super::utils::start_day;
use std::collections::HashSet;

pub fn main() {
    let input = start_day("eight");

    let code = Code::parse(&input);

    println!("Part one: {}", part_one(&code));
    println!("Part two: {}", part_two(&code));
    println!();
}

fn part_one(code: &Code) -> isize {
    let mut already_visited = HashSet::new();

    let mut prev_acc = 0;

    for (index, acc) in code.new_program() {
        if !already_visited.insert(index) {
            return prev_acc;
        }

        prev_acc = acc;
    }

    panic!("Solution not found");
}

fn part_two(code: &Code) -> isize {
    let mut i = 0;

    loop {
        let mut new_code = code.clone();
        i = new_code.mutate_instruction(i) + 1;

        if let Some(acc) = check_program_works(new_code.new_program()) {
            return acc;
        }
    }
}

fn check_program_works(program: Program) -> Option<isize> {
    let mut already_visited = HashSet::new();
    let mut prev_acc = 0;

    for (index, acc) in program {
        if !already_visited.insert(index) {
            return None;
        }

        prev_acc = acc;
    }

    Some(prev_acc)
}

#[derive(Debug, Copy, Clone)]
enum Instr {
    NoOp,
    Acc,
    Jump,
}

impl Instr {
    pub fn parse(s: &str) -> Self {
        match s {
            "nop" => Instr::NoOp,
            "acc" => Instr::Acc,
            "jmp" => Instr::Jump,
            _ => panic!("Unknown instruction: {}", s),
        }
    }
}

#[derive(Debug, Clone)]
struct Code(Vec<(Instr, isize)>);

impl Code {
    pub fn parse(input: &str) -> Self {
        let code = input
            .lines()
            .map(|l| {
                let instr = Instr::parse(&l[..3]);
                let x = (&l[4..]).parse().unwrap();

                (instr, x)
            })
            .collect();

        Code(code)
    }

    pub fn new_program(&self) -> Program {
        Program {
            code: self,
            acc: 0,
            pos: 0,
        }
    }

    pub fn mutate_instruction(&mut self, start_from: usize) -> usize {
        for (j, &(instr, x)) in (&self.0[start_from..]).iter().enumerate() {
            let i = start_from + j;

            match instr {
                Instr::Jump => {
                    self.0[i] = (Instr::NoOp, x);
                    return i;
                }
                Instr::NoOp => {
                    self.0[i] = (Instr::Jump, x);
                    return i;
                }
                _ => {}
            }
        }

        panic!()
    }
}

#[derive(Debug, Clone)]
struct Program<'a> {
    code: &'a Code,
    acc: isize,
    pos: usize,
}

impl<'a> Iterator for Program<'a> {
    type Item = (usize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.code.0.get(self.pos)? {
            (Instr::NoOp, _) => {
                self.pos += 1;
            }
            (Instr::Acc, x) => {
                self.acc += x;
                self.pos += 1;
            }
            (Instr::Jump, x) => {
                // Could overflow
                let abs = x.abs() as usize;

                if x.is_negative() {
                    self.pos -= abs;
                } else {
                    self.pos += abs;
                }
            }
        };

        Some((self.pos, self.acc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
    ";

    #[test]
    fn sample_input_part_one() {
        let code = Code::parse(SAMPLE_INPUT.trim());

        assert_eq!(part_one(&code), 5);
    }

    #[test]
    fn sample_input_part_two() {
        let code = Code::parse(SAMPLE_INPUT.trim());

        assert_eq!(part_two(&code), 8);
    }
}
