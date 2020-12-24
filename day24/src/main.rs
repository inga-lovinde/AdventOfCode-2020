use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut changed_tiles = HashSet::new();
    for line_result in stdin.lock().lines() {
        let processed_line = line_result.unwrap().replace("nw", "7").replace("ne", "9").replace("sw", "1").replace("se", "3");
        let mut x = 0;
        let mut y = 0;
        for ch in processed_line.chars() {
            x += match ch {
                'e' => 2,
                'w' => -2,
                '9' | '3' => 1,
                '7' | '1' => -1,
                _ => 0,
            };
            y += match ch {
                '9' | '7' => 1,
                '3' | '1' => -1,
                _ => 0,
            };
        }

        let tile = (x, y);
        if changed_tiles.contains(&tile) {
            changed_tiles.remove(&tile);
        } else {
            changed_tiles.insert(tile);
        }
    }

    println!("{}", changed_tiles.len());
}
