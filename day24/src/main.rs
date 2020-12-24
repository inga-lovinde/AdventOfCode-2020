use std::collections::HashSet;
use std::io::{self, BufRead};

mod game;
mod locations;

use crate::game::Game;

const MAX_NUMBER_OF_STEPS: usize = 100;

fn main() {
    let stdin = io::stdin();
    let mut alive_tiles = HashSet::new();
    for line_result in stdin.lock().lines() {
        let processed_line = line_result.unwrap().replace("nw", "n").replace("se", "s");
        let mut x = 0i16;
        let mut y = 0i16;
        for ch in processed_line.chars() {
            x += match ch {
                'e' => 1,
                'w' => -1,
                _ => 0,
            };
            y += match ch {
                'n' => 1,
                's' => -1,
                _ => 0,
            };
        }

        let tile = (x, y);
        if alive_tiles.contains(&tile) {
            alive_tiles.remove(&tile);
        } else {
            alive_tiles.insert(tile);
        }
    }

    let mut game = Game::from_input(&alive_tiles.iter().cloned().collect::<Vec<_>>(), MAX_NUMBER_OF_STEPS);

    println!("{}", game.get_alive_count());

    for i in 1..=MAX_NUMBER_OF_STEPS {
        game.next_step();
        println!("Day {}: {}", i, game.get_alive_count());
    }
}
