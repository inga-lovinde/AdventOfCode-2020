use std::collections::HashSet;
use std::io::{self, BufRead};

// Could be solved in O(n)
// (sort two copies of the list and walk it from the opposite ends to find a match)
// But O(n*log(n)) solution is much simpler.
fn main() {
    let stdin = io::stdin();
    let source_numbers: Vec<_> = stdin.lock().lines().into_iter()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    let first_set: HashSet<_> = source_numbers.iter().map(|&number| number).collect();
    let second_set: HashSet<_> = source_numbers.iter().map(|&number| 2020-number).collect();

    let result = first_set.intersection(&second_set);
    for value in result {
        println!("{}", value * (2020-value));
    }
}
