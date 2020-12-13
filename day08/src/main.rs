use std::collections::HashSet;
use std::io::{self, BufRead};
use std::error::Error;
use std::str::FromStr;

#[macro_use] extern crate lazy_static;
use regex::Regex;

#[derive(Clone, Copy)]
enum Instruction {
    Accumulate(i64),
    Jump(i64),
    NoOp(i64),
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"^(\w+)\s+([+-]?\d+)$").unwrap();
        }

        match LINE_RE.captures(s) {
            Some(captures) => {
                match captures[1].as_ref() {
                    "acc" => Ok(Self::Accumulate(captures[2].parse()?)),
                    "jmp" => Ok(Self::Jump(captures[2].parse()?)),
                    "nop" => Ok(Self::NoOp(captures[2].parse()?)),
                    _ => Err(Box::from("wrong command")),
                }
            },
            _ => Err(Box::from("wrong string format"))
        }
    }
}

enum ProgramResult {
    Terminated(i64),
    Looped(i64),
}

struct Program<'a> {
    instructions: &'a[Instruction],
    override_index: i64,
}

impl<'a> Program<'a> {
    pub fn new(instructions: &'a [Instruction], override_index: i64) -> Self {
        Program {
            instructions,
            override_index,
        }
    }

    fn get_instruction(&self, index: i64) -> Option<Instruction> {
        if index < 0 || (index as usize) >= self.instructions.len() {
            return None;
        }

        if index != self.override_index {
            return Some(self.instructions[index as usize]);
        }

        match self.instructions[index as usize] {
            Instruction::Accumulate(value) => Some(Instruction::Accumulate(value)),
            Instruction::Jump(ignore) => Some(Instruction::NoOp(ignore)),
            Instruction::NoOp(offset) => Some(Instruction::Jump(offset)),
        }
    }

    pub fn run(&self) -> ProgramResult {
        let mut current_instruction_index = 0;
        let mut visited_instructions = HashSet::new();
        let mut accumulator = 0;
        loop {
            if visited_instructions.contains(&current_instruction_index) {
                return ProgramResult::Looped(accumulator);
            }
    
            visited_instructions.insert(current_instruction_index);
    
            match self.get_instruction(current_instruction_index) {
                Some(Instruction::Accumulate(value)) => {
                    accumulator += value;
                },
                Some(Instruction::Jump(offset)) => {
                    current_instruction_index += offset-1;
                }
                Some(Instruction::NoOp(_)) => {},
                None => {
                    return ProgramResult::Terminated(accumulator);
                }
            }
            
            current_instruction_index += 1;
        }
        }
}

fn main() {
    let stdin = io::stdin();
    let instructions: Vec<_> = stdin.lock().lines().into_iter()
        .map(|line| line.unwrap().parse::<Instruction>().unwrap())
        .collect();

    let program = Program::new(&instructions, -1);
    match program.run() {
        ProgramResult::Looped(accumulator) => println!("Original program looped: {}", accumulator),
        ProgramResult::Terminated(accumulator) => println!("Original program terminated: {}", accumulator),
    }

    for i in 0..instructions.len() {
        let program = Program::new(&instructions, i as i64);
        match program.run() {
            ProgramResult::Looped(_) => {},
            ProgramResult::Terminated(accumulator) => println!("Program #{} terminated: {}", i, accumulator),
        }
    }
}
