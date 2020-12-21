use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut answers = HashMap::new();
    let mut result_anybody = 0;
    let mut result_everybody = 0;
    let mut group_size = 0;
    for line_result in stdin.lock().lines() {
        let line = line_result.unwrap();
        if line == "" {
            let group_anybody = answers.len();
            let group_everybody = answers.values().filter(|&&count| count == group_size).count();
            result_anybody += group_anybody;
            result_everybody += group_everybody;
            println!("Group: {}/{}, total: {}/{}", group_everybody, group_anybody, result_everybody, result_anybody);
            answers.clear();
            group_size = 0;
        } else {
            group_size += 1;
            for ch in line.chars() {
                let count = answers.entry(ch).or_insert(0);
                *count += 1;
            }
        }
    }
}
