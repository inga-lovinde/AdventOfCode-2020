use std::io::{self, BufRead};
use std::error::Error;
use std::str::FromStr;

#[macro_use] extern crate lazy_static;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Mask {
    and: u64,
    or: u64,
}

impl Mask {
    fn from(and: u64, or: u64) -> Mask {
        Mask {
            and,
            or,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    SetMask(Mask),
    SetValue{ address: usize, value: u64 },
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MASK_RE: Regex = Regex::new(r"^mask\s*=\s*([01X]+)$").unwrap();
            static ref MEM_RE: Regex = Regex::new(r"^mem\[(\d+)\]\s*=\s*(\d+)$").unwrap();
        }

        match (MASK_RE.captures(s), MEM_RE.captures(s)) {
            (Some(mask_captures), None) => Ok(Self::SetMask(Mask::from(
                u64::from_str_radix(&mask_captures[1].replace("X", "1"), 2)?,
                u64::from_str_radix(&mask_captures[1].replace("X", "0"), 2)?,
            ))),
            (None, Some(mem_re)) => Ok(Self::SetValue {
                address: mem_re[1].parse()?,
                value: mem_re[2].parse()?,
            }),
            _ => Err(Box::from("Wrong string format")),
        }
    }
}

struct ProgramState {
    mask: Mask,
    memory: Vec<u64>,
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState {
            mask: Mask::from(!0, 0),
            memory: vec![0; 1 << 16],
        }
    }

    pub fn apply_instruction(&mut self, instruction: Instruction) -> () {
        match instruction {
            Instruction::SetMask(mask) => self.mask = mask,
            Instruction::SetValue{ address, value } => self.memory[address] = (value | self.mask.or) & self.mask.and,
        }
    }

    pub fn get_sum(&self) -> u64 {
        self.memory.iter().sum()
    }
}

fn main() {
    let stdin = io::stdin();
    let instructions: Vec<_> = stdin.lock().lines().into_iter()
        .map(|line| line.unwrap().parse::<Instruction>().unwrap())
        .collect();

    let mut program_state = ProgramState::new();
    for instruction in instructions {
        program_state.apply_instruction(instruction);
    }

    println!("{}", program_state.get_sum());
}
