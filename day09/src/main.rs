use std::collections::HashMap;
use std::io::{self, BufRead};

const BLOCK_SIZE: usize = 25;

fn main() {
    let stdin = io::stdin();
    let numbers: Vec<u128> = stdin.lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut part1_key = 0;
    for i in BLOCK_SIZE..numbers.len() {
        let mut found: bool = false;
        for j in i-BLOCK_SIZE..i {
            for k in j+1..i {
                if numbers[j] + numbers[k] == numbers[i] {
                    found = true;
                }
            }
        }

        if !found {
            part1_key = i;
            break;
        }
    }

    println!("{}", numbers[part1_key]);

    for i in 0..part1_key {
        let mut sum = 0u128;
        let mut smallest = u128::MAX;
        let mut largest = u128::MIN;
        for j in i..part1_key {
            sum += numbers[j];
            smallest = u128::min(smallest, numbers[j]);
            largest = u128::max(largest, numbers[j]);
            if sum == numbers[part1_key] {
                println!("{} ({},{})", smallest + largest, i, j);
                return;
            }
        }

    }
}
