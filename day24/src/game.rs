use ndarray::Array2;

use crate::locations::{CellLocation, MINICUBE_DIFF_ARRAY};

struct CellInfo {
    is_alive: bool, // alive = black, dead = white
    alive_minicube_count: usize,
}

impl Default for CellInfo {
    fn default() -> CellInfo {
        CellInfo {
            is_alive: false,
            alive_minicube_count: 0,
        }
    }
}

impl CellInfo {
    pub fn set_alive(&mut self) {
        self.is_alive = true;
    }

    pub fn add_alive_minicube(&mut self) {
        self.alive_minicube_count += 1;
    }

    pub fn set_dead(&mut self) {
        self.is_alive = false;
    }

    pub fn add_dead_minicube(&mut self) {
        self.alive_minicube_count -= 1;
    }
}

pub struct Game {
    board: Array2<CellInfo>,
    corner: CellLocation,
}

impl Game {
    fn make_alive(&mut self, cell_location: CellLocation) {
        self.board[cell_location].set_alive();
        for &diff in &MINICUBE_DIFF_ARRAY {
            self.board[cell_location + diff].add_alive_minicube();
        }
    }

    fn make_dead(&mut self, cell_location: CellLocation) {
        self.board[cell_location].set_dead();
        for &diff in &MINICUBE_DIFF_ARRAY {
            self.board[cell_location + diff].add_dead_minicube();
        }
    }

    pub fn next_step(&mut self) -> usize {
        let mut new_alive: Vec<_> = vec![];
        let mut new_dead: Vec<_> = vec![];
        for x in 0..self.corner.x {
            for y in 0..self.corner.y {
                let location = CellLocation { x, y };
                match &self.board[location] {
                    CellInfo { is_alive: true, alive_minicube_count } if *alive_minicube_count == 0 || *alive_minicube_count > 2 => {
                        new_dead.push(location);
                    },
                    CellInfo { is_alive: false, alive_minicube_count: 2 } => {
                        new_alive.push(location);
                    },
                    _ => {}
                }
            }
        }

        let changes_count = new_alive.len() + new_dead.len();
        for location in new_alive {
            self.make_alive(location);
        }
        for location in new_dead {
            self.make_dead(location);
        }

        changes_count
    }

    pub fn from_input(input_data: &[(i16, i16)], max_number_of_steps: usize) -> Self {
        let min_x = input_data.iter().map(|(x, _y)| *x).min().unwrap();
        let max_x = input_data.iter().map(|(x, _y)| *x).max().unwrap();
        let min_y = input_data.iter().map(|(_x, y)| *y).min().unwrap();
        let max_y = input_data.iter().map(|(_x, y)| *y).max().unwrap();
        let input_size_x = max_x - min_x;
        let input_size_y = max_y - min_y;
        let size_x = (input_size_x as usize) + max_number_of_steps * 2;
        let size_y = (input_size_y as usize) + max_number_of_steps * 2;
        let offset_x = (max_number_of_steps as i16) - min_x;
        let offset_y = (max_number_of_steps as i16) - min_y;
        let corner = CellLocation {
            x: size_x,
            y: size_y,
        };
        let board = Array2::default((corner.x, corner.y));
        let mut game = Game {
            board,
            corner,
        };

        for (input_x, input_y) in input_data {
            game.make_alive(CellLocation {
                x: (input_x + offset_x) as usize,
                y: (input_y + offset_y) as usize,
            });
        }

        return game;
    }

    pub fn get_alive_count(&self) -> usize {
        (&self.board).iter().filter(|&cell| cell.is_alive).count()
    }
}

