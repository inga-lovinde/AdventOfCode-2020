use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut joltages: Vec<usize> = stdin.lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    joltages.push(0);
    joltages.push(joltages.iter().max().unwrap() + 3);

    joltages.sort();

    let mut differences = HashMap::new();
    for i in 1..joltages.len() {
        let count = differences.entry(joltages[i] - joltages[i-1]).or_insert(0);
        *count += 1;
    }

    for (key, value) in differences.iter() {
        println!("{}: {}", key, value);
    }

    println!("{}", differences.get(&1).unwrap_or(&0) * differences.get(&3).unwrap_or(&0));

    let mut arrangements = vec![0u128; joltages.len()];
    arrangements[0] = 1;
    for i in 1..joltages.len() {
        for j in 0..i {
            if (joltages[j] + 3) >= joltages[i] {
                arrangements[i] += arrangements[j];
            }
        }
    }

    println!("{}", arrangements[joltages.len() - 1]);
}
