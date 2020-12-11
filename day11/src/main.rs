#![feature(trait_alias)]

use std::io::{self, BufRead};

mod binary;
mod board_metadata;
mod game;
mod rules;
mod rules_easy;

use binary::State;
use game::Game;
use rules_easy::RulesEasy;

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|line| line.unwrap()).collect();
    let mut game = Game::from_input::<RulesEasy>(&lines);

    //game.print_board();

    for i in 1.. {
        let changes_count = game.next_step();
        println!("Iteration {}; changed cells: {}", i, changes_count);
        //game.print_board();
        if changes_count == 0 {
            break;
        }
    }

    game.print_board();
    println!("Board stabilized at {} occupied seats", game.get_count_of_cells_for_state(State::SeatOccupied));
}
