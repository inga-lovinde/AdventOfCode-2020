use std::io::{self, BufRead};

mod game;
mod locations;

use crate::game::{Game, MAX_NUMBER_OF_STEPS};

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|line| line.unwrap()).collect();

    let mut game = Game::from_input(&lines);

    for i in 1..MAX_NUMBER_OF_STEPS {
        let changes_count = game.next_step();
        println!("Iteration {}; changed cells: {}; alive_cells: {}", i, changes_count, game.get_alive_count());
        if changes_count == 0 {
            break;
        }
    }
}
