use std::cmp::Reverse;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug)]
struct Condition {
    pub base: u128,
    pub remainder: u128,
}

impl Condition {
    fn new(base: u128, remainder: u128) -> Condition {
        Condition {
            base,
            remainder,
        }
    }
}

impl Default for Condition {
    fn default() -> Self {
        Self {
            base: 1,
            remainder: 0,
        }
    }
}

// chinese remainder theorem, sieve method
// assuming all bases are coprime
fn solve(current: Condition, remaining_conditions: &[Condition]) -> Option<Condition> {
    if remaining_conditions.len() == 0 {
        return Some(current);
    }

    let first_condition = remaining_conditions[0];
    for i in 0..first_condition.base {
        if (i * current.base + current.remainder) % first_condition.base == first_condition.remainder {
            return solve(Condition::new(current.base * first_condition.base, i * current.base + current.remainder), &remaining_conditions[1..]);
        }
    }

    return None;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lines = stdin.lock().lines();
    let _timestamp: u128 = stdin_lines.next().unwrap().unwrap().parse().unwrap();
    let mut conditions: Vec<_> = stdin_lines.next().unwrap().unwrap().split(",")
        .enumerate()
        .filter(|(_index, entry)| *entry != "x")
        .map(|(index, entry)| (entry.parse::<u128>().unwrap(), index))
        .map(|(base, index)| Condition::new(base, (-(index as i128)).rem_euclid(base as i128) as u128))
        .collect();

    conditions.sort_by_key(|condition| Reverse(condition.base));

    for condition in conditions.iter() {
        println!("{:?}", condition);
    }

    let solution = solve(Condition::default(), &conditions).unwrap();
    println!("{:?}", solution);
}
