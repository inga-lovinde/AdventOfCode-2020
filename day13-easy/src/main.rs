use std::io::{self, BufRead};

fn get_wait_time(timestamp: u64, bus_number: u64) -> u64 {
    let remainder = timestamp % bus_number;
    if remainder == 0 {
        return 0;
    }

    bus_number - remainder
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_lines = stdin.lock().lines();
    let timestamp: u64 = stdin_lines.next().unwrap().unwrap().parse().unwrap();
    let earliest_bus_number = stdin_lines.next().unwrap().unwrap().split(",")
        .filter(|&entry| entry != "x")
        .map(|entry| entry.parse().unwrap())
        .min_by_key(|&bus_number| get_wait_time(timestamp, bus_number))
        .unwrap();

    println!("{}", earliest_bus_number * get_wait_time(timestamp, earliest_bus_number));
}
