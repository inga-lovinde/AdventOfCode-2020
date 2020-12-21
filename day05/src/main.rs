use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut tickets: Vec<u16> = Vec::new();
    for line_result in stdin.lock().lines() {
        let binary_line = line_result.unwrap().replace('F', "0").replace('B', "1").replace('L', "0").replace('R', "1");
        tickets.push(u16::from_str_radix(&binary_line, 2).unwrap());
    }

    println!("max: {}", tickets.iter().max().unwrap());

    tickets.sort();
    for i in 1..tickets.len() {
        if tickets[i] - tickets[i-1] != 1 {
            println!("hole: {}-{}", tickets[i-1], tickets[i]);
        }
    }
}
