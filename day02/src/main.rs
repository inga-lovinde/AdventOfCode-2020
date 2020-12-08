use std::io::{self, BufRead};
use std::error::Error;
use std::str::FromStr;

#[macro_use] extern crate lazy_static;
use regex::Regex;

struct LineInfo {
    first_number: usize,
    second_number: usize,
    ch: char,
    password: String,
}

impl LineInfo {
    fn is_valid_easy(&self) -> bool {
        let count = self.password.chars().filter(|&password_char| password_char == self.ch).count();
        return self.first_number <= count && count <= self.second_number;
    }

    fn is_valid_hard(&self) -> bool {
        let chars: Vec<_> = self.password.chars().collect();
        return (chars[self.first_number-1] == self.ch) ^ (chars[self.second_number-1] == self.ch);
    }
}

impl FromStr for LineInfo {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"^(\d+)-(\d+)\s+([\S]):\s(.*)$").unwrap();
        }

        match LINE_RE.captures(s) {
            Some(captures) => {
                Ok(LineInfo {
                    first_number: captures[1].parse()?,
                    second_number: captures[2].parse()?,
                    ch: captures[3].chars().next().unwrap(),
                    password: captures[4].to_owned(),
                })
            },
            _ => Err(Box::from("wrong string format"))
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().into_iter()
        .map(|line| line.unwrap().parse::<LineInfo>().unwrap())
        .collect();

    println!("{}", lines.iter().filter(|line_info| line_info.is_valid_easy()).count());
    println!("{}", lines.iter().filter(|line_info| line_info.is_valid_hard()).count());

    for line_info in lines {
        println!("easy {}, hard {}: {}-{} {}: {}", line_info.is_valid_easy(), line_info.is_valid_hard(), line_info.first_number, line_info.second_number, line_info.ch, line_info.password);
    }
}
