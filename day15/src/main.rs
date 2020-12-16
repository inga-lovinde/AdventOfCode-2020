use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut stdin_lines = stdin.lock().lines();
    let seed: Vec<usize> = stdin_lines.next().unwrap().unwrap().split(",")
        .map(|entry| entry.parse().unwrap())
        .collect();

    let mut last_occurrences = HashMap::new();
    for i in 0..seed.len()-1 {
        last_occurrences.insert(seed[i], i+1);
    }

    let mut last_number = seed[seed.len()-1];
    for turn in seed.len()+1.. {
        let next_number = match last_occurrences.get(&last_number) {
            Some(&last_occurrence) => (turn-1) - last_occurrence,
            None => 0,
        };

        last_occurrences.insert(last_number, turn-1);
        last_number = next_number;
        if turn == 2020 || (turn % 1_000_000) == 0 {
            println!("{}: {}", turn, last_number);
        }
    }
}
