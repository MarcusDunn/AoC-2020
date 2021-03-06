use core::fmt;
use std::convert::TryInto;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operation::Acc => {
                    "acc"
                }
                Operation::Jmp => {
                    "jmp"
                }
                Operation::Nop => {
                    "nop"
                }
            }
        )
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(Operation::Nop),
            "acc" => Ok(Operation::Acc),
            "jmp" => Ok(Operation::Jmp),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    operation: Operation,
    argument: i64,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.operation, self.argument)
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (op, arg) = s
            .split_once(" ")
            .unwrap_or_else(|| panic!("called split_once(\" \") on {}", s));
        Ok(Instruction {
            operation: op
                .parse()
                .unwrap_or_else(|_| panic!("called split_once(\" \") on {}", s)),
            argument: arg
                .parse()
                .unwrap_or_else(|_| panic!("attempted to turn {} into a Operation", op)),
        })
    }
}

#[derive(Debug)]
pub struct Program {
    accumulator: i64,
    program_counter: usize,
    instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Program {
        Program {
            accumulator: 0,
            program_counter: 0,
            instructions,
        }
    }

    pub fn get_acc(&self) -> i64 {
        self.accumulator
    }

    fn fetch(&self) -> Option<&Instruction> {
        self.instructions.get(self.program_counter as usize)
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction.operation {
            Operation::Acc => {
                self.accumulator += instruction.argument;
                self.program_counter += 1;
            }
            Operation::Jmp => {
                self.program_counter = (self.program_counter as i64 + instruction.argument)
                    .try_into()
                    .unwrap()
            }
            Operation::Nop => {
                self.program_counter += 1;
            }
        }
    }

    fn step(&mut self) -> Option<()> {
        if let Some(&instruction) = self.fetch() {
            self.execute(instruction);
            Some(())
        } else {
            None
        }
    }

    pub fn fix(&mut self) {
        let mut i = 0;
        loop {
            self.swap_jmp_nop(i);
            if self.loops() {
                self.swap_jmp_nop(i);
                self.reset();
                i += 1;
            } else {
                break;
            }
        }
    }

    fn reset(&mut self) {
        self.accumulator = 0;
        self.program_counter = 0;
    }

    fn swap_jmp_nop(&mut self, i: usize) {
        match self.instructions[i].operation {
            Operation::Acc => {}
            Operation::Jmp => self.instructions[i].operation = Operation::Nop,
            Operation::Nop => self.instructions[i].operation = Operation::Jmp,
        }
    }

    pub fn loops(&mut self) -> bool {
        let mut executed = vec![];
        loop {
            self.step();
            let pc = self.program_counter;
            if executed.contains(&pc) {
                return true;
            } else if self.program_counter == self.instructions.len() {
                return false;
            } else {
                executed.push(pc)
            }
        }
    }

    pub fn run_until_loop(&mut self) {
        let mut executed = vec![];
        loop {
            self.step();
            let pc = self.program_counter;
            if executed.contains(&pc) {
                break;
            } else {
                executed.push(pc)
            }
        }
    }
}
impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "acc: {}, pc: {}", self.accumulator, self.program_counter)
    }
}

#[cfg(test)]
mod test {
    use crate::day08::{Instruction, Program};
    use crate::loader::file_to_vec;

    #[test]
    fn test_parse() {
        file_to_vec::<Instruction>("inputs/day08small.txt");
    }

    #[test]
    fn test_program_small() {
        let mut prgm = Program::new(file_to_vec::<Instruction>("inputs/day08small.txt"));
        prgm.run_until_loop();
        assert_eq!(5, prgm.get_acc());
    }

    #[test]
    fn test_program() {
        let mut prgm = Program::new(file_to_vec::<Instruction>("inputs/day08.txt"));
        prgm.run_until_loop();
        assert_eq!(1475, prgm.get_acc());
    }

    #[test]
    fn test_loops_true() {
        let mut prgm = Program::new(file_to_vec::<Instruction>("inputs/day08small.txt"));
        assert!(prgm.loops());
    }

    #[test]
    fn test_loops_false() {
        let mut prgm = Program::new(file_to_vec::<Instruction>("inputs/day08small_no_loop.txt"));
        assert!(!prgm.loops());
    }

    #[test]
    fn test_fix_small() {
        let mut prgm = Program::new(file_to_vec::<Instruction>("inputs/day08small.txt"));
        prgm.fix();
        assert_eq!(8, prgm.get_acc())
    }

    #[test]
    fn test_fix() {
        let mut prgm = Program::new(file_to_vec::<Instruction>("inputs/day08.txt"));
        prgm.fix();
        assert_eq!(1270, prgm.get_acc())
    }
}
