use std::io::{self, BufRead};

// non-negative numbers are presorted, largest first
fn check(numbers: &[u32], remaining_count: u32, remaining_sum: u32, current_product: u64) -> () {
    if remaining_count == 0 {
        if remaining_sum == 0 {
            println!("{}", current_product);
        }

        return;
    }

    for (i, &number) in numbers.iter().enumerate() {
        if number * remaining_count < remaining_sum {
            return;
        }

        if number <= remaining_sum {
            check(&numbers[i..], remaining_count - 1, remaining_sum - number, current_product * (number as u64));
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut source_numbers: Vec<_> = stdin.lock().lines().into_iter()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect();

    source_numbers.sort();
    source_numbers.reverse();

    check(&source_numbers, 3, 2020, 1);
}
