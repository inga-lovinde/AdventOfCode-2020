#![feature(trait_alias)]

use std::io::{self, BufRead};
use ndarray::Array2;

// state (u32 for simplicity reasons): 0/1/2/3 (in this case, 1 = floor, 2 = seat free, 3 = seat taken)

// every cell is u32, where:
// * lowest 16 bits describe its neighbours:
//     * bits 0-1 (cell & 3) is the state of upper left neighbour,
//     * bits 2-3 ((cell >> 2) & 3) is the state of upper neighbour,
//     * bits 4-5 is the state of upper right neighbour,
//     * bits 6-7, left,
//     * bits 8-9, right,
//     * bits 10-11, bottom left,
//     * bits 12-13, bottom,
//     * bits 14-15, bottom right
// * bits 16-17 ((cell >> 16) & 3) describe the cell itself

// rule: cell (u32, 18 bits used) -> new cell (u32)

// board is indexed by [row, column],
// where row is from top to bottom and column is from left to right

trait StateRules = Fn([usize; 4], u32) -> u32;

struct Game {
    cell_rules: Vec<u32>,
    board: Array2<u32>,
    rows: usize,
    columns: usize,
}

impl Game {
    // only updates the state of this cell for it and its neighbours;
    // only state of this cell is used from new_cell
    fn update_cell(&mut self, row: usize, column: usize, new_cell: u32) {
        let state_diff = (new_cell ^ self.board[[row, column]]) >> 16;

        self.board[[row, column]] ^= state_diff << 16;

        self.board[[row+1, column+1]] ^= state_diff;
        self.board[[row+1, column  ]] ^= state_diff <<  2;
        self.board[[row+1, column-1]] ^= state_diff <<  4;
        self.board[[row  , column+1]] ^= state_diff <<  6;
        self.board[[row  , column-1]] ^= state_diff <<  8;
        self.board[[row-1, column+1]] ^= state_diff << 10;
        self.board[[row-1, column  ]] ^= state_diff << 12;
        self.board[[row-1, column-1]] ^= state_diff << 14;
    }

    fn build_cell_rules<T: StateRules>(state_rules: T) -> Vec<u32> {
        let mut result = vec![0u32; 1 << 18];

        for i in 0..1usize << 18 {
            let cell = i as u32;
            let current_state = (cell >> 16) & 3;
            let mut state_counts = [0usize; 4];
            for j in 0..8 {
                state_counts[((cell >> (2*j)) & 3) as usize] += 1;
            }

            let new_state = state_rules(state_counts, current_state);
            let new_cell = cell ^ ((current_state ^ new_state) << 16);
            result[i] = new_cell;
        }

        result
    }

    pub fn next_step(&mut self) -> usize {
        let mut changes: Vec<_> = vec![];
        for row in 1..self.rows-1 {
            for column in 1..self.columns-1 {
                let old_cell = self.board[[row, column]];
                let new_cell = self.cell_rules[old_cell as usize];
                if new_cell != old_cell {
                    changes.push((row, column, new_cell));
                }
            }
        }

        let changes_count = changes.len();
        for (row, column, new_cell) in changes {
            self.update_cell(row, column, new_cell);
        }

        changes_count
    }

    pub fn from_input<T: StateRules>(input_data: &[String], state_rules: T) -> Game {
        let rows = input_data.len() + 2;
        let columns = input_data[0].len() + 2;
        let mut states = Array2::zeros((rows, columns));

        for row in 1..rows-1 {
            let chars = input_data[row-1].chars().collect::<Vec<_>>();
            for column in 1..columns-1 {
                let ch = chars[column-1];
                states[[row, column]] = match ch {
                    '.' => 1,
                    'L' => 2,
                    '#' => 3,
                    _ => 0,
                }
            }
        }

        let mut board = Array2::zeros((rows, columns));

        /*
        board[[0, 0]] = states[[1, 1]] << 14;
        board[[0, columns-1]] = states[[1, columns-2]] << 10;
        board[[rows-1, 0]] = states[[rows-2, 1]] << 4;
        board[[rows-1, columns-1]] = states[[rows-2, columns-2]];

        for row in 1..rows-1 {
            board[[row, 0]] = (states[[row-1, 1]] << 4) ^ (states[[row, 1]] << 8) ^ (states[[row+1, 1]] << 14);
            board[[row, columns-1]] = (states[[row-1, columns-2]]) ^ (states[[row, columns-2]] << 6) ^ (states[[row+1, columns-2]] << 10);
        }

        for column in 1..columns-1 {
            board[[0, column]] = (states[[1, column-1]] << 10) ^ (states[[1, column]] << 12) ^ (states[[1, column+1]] << 14);
            board[[rows-1, column]] = (states[[rows-2, column-1]]) ^ (states[[rows-2, column]] << 2) ^ (states[[rows-2, column+1]] << 4);
        }
        */

        for row in 1..rows-1 {
            for column in 1..columns-1 {
                board[[row, column]] =
                    (states[[row-1, column-1]]      ) ^
                    (states[[row-1, column  ]] <<  2) ^
                    (states[[row-1, column+1]] <<  4) ^
                    (states[[row  , column-1]] <<  6) ^
                    (states[[row  , column+1]] <<  8) ^
                    (states[[row+1, column-1]] << 10) ^
                    (states[[row+1, column  ]] << 12) ^
                    (states[[row+1, column+1]] << 14) ^
                    (states[[row, column]] << 16);
            }
        }

        let cell_rules = Self::build_cell_rules(state_rules);

        Game {
            rows,
            columns,
            board,
            cell_rules,
        }
    }

    pub fn print_board(&self) {
        for row in (&self.board).genrows() {
            println!("{}", row.iter().map(|state| {
                match (state >> 16) & 3 {
                    1 => '.',
                    2 => 'L',
                    3 => '#',
                    _ => '0',
                }
            }).collect::<String>());
        }
    }

    pub fn get_count_of_cells_for_state(&self, state: u32) -> usize {
        (&self.board).iter().filter(|&&cell| ((cell >> 16) & 3) == state).count()
    }
}

fn main() {
    let stdin = io::stdin();
    let lines: Vec<_> = stdin.lock().lines().map(|line| line.unwrap()).collect();
    let mut game = Game::from_input(&lines, |state_counts: [usize; 4], current_state| {
        match current_state {
            2 => if state_counts[3] == 0 { 3 } else { 2 },
            3 => if state_counts[3] >= 4 { 2 } else { 3 },
            other => other
        }
    });

    for i in 1.. {
        let changes_count = game.next_step();
        println!("Iteration {}; changed cells: {}", i, changes_count);
        if changes_count == 0 {
            break;
        }
    }

    println!("Board stabilized at {} occupied seats", game.get_count_of_cells_for_state(3));
    game.print_board();
    println!("Board stabilized at {} occupied seats", game.get_count_of_cells_for_state(3));
}
