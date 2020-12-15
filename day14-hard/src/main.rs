use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead};
use std::str::FromStr;

#[macro_use] extern crate lazy_static;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Mask {
    pub bypass: u64,
    pub floating: u64,
    pub ones: u64,
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    SetMask(Mask),
    SetValue{ address: u64, value: u128 },
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MASK_RE: Regex = Regex::new(r"^mask\s*=\s*([01X]+)$").unwrap();
            static ref MEM_RE: Regex = Regex::new(r"^mem\[(\d+)\]\s*=\s*(\d+)$").unwrap();
        }

        match (MASK_RE.captures(s), MEM_RE.captures(s)) {
            (Some(mask_captures), None) => Ok(Self::SetMask(Mask {
                bypass: u64::from_str_radix(&mask_captures[1].replace("1", "X").replace("0", "1").replace("X", "0"), 2)?,
                floating: u64::from_str_radix(&mask_captures[1].replace("1", "0").replace("X", "1"), 2)?,
                ones: u64::from_str_radix(&mask_captures[1].replace("X", "0"), 2)?,
            })),
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
    memory: HashMap<u64, u128>,
}

impl ProgramState {
    pub fn new() -> ProgramState {
        ProgramState {
            mask: Mask {
                bypass: !0,
                floating: 0,
                ones: 0,
            },
            memory: HashMap::new(),
        }
    }

    pub fn apply_instruction(&mut self, instruction: Instruction) -> () {
        match instruction {
            Instruction::SetMask(mask) => self.mask = mask,
            Instruction::SetValue{ address, value } => {
                let address_min = (address & self.mask.bypass) | self.mask.ones;
                let address_max = address_min | self.mask.floating;
                for i in address_min..=address_max { // insanely slow, could be optimized, but I'm lazy :(
                    if (i & address_min) == address_min && (i & address_max) == i {
                        self.memory.insert(i, value);
                        println!("Inserting {} into {}", value, i);
                    }
                }
            }
        }
    }

    pub fn get_sum(&self) -> u128 {
        self.memory.values().sum()
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
        println!("Processed instruction");
    }

    println!("{}", program_state.get_sum());
}
