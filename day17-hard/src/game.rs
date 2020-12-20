use ndarray::Array4;

use crate::locations::{CellLocation, MINICUBE_DIFF_ARRAY};

struct CellInfo {
    is_alive: bool,
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

pub const MAX_NUMBER_OF_STEPS: usize = 30;

pub struct Game {
    board: Array4<CellInfo>,
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
                for z in 0..self.corner.z {
                    for w in 0..self.corner.w {
                        let location = CellLocation { x, y, z, w };
                        match &self.board[location] {
                            CellInfo { is_alive: true, alive_minicube_count } if *alive_minicube_count < 3 || *alive_minicube_count > 4 => {
                                new_dead.push(location);
                            },
                            CellInfo { is_alive: false, alive_minicube_count: 3 } => {
                                new_alive.push(location);
                            },
                            _ => {}
                        }
                    }
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

    pub fn from_input(input_data: &[String]) -> Self {
        let rows = input_data.len();
        let columns = input_data[0].len();
        let corner = CellLocation {
            x: rows + 2*MAX_NUMBER_OF_STEPS,
            y: columns + 2*MAX_NUMBER_OF_STEPS,
            z: 1 + 2*MAX_NUMBER_OF_STEPS,
            w: 1 + 2*MAX_NUMBER_OF_STEPS,
        };
        let board = Array4::default((corner.x, corner.y, corner.z, corner.w));
        let mut game = Game {
            board,
            corner,
        };

        for row in 0..rows {
            let chars = input_data[row].chars().collect::<Vec<_>>();
            for column in 0..columns {
                let ch = chars[column];
                if ch == '#' {
                    game.make_alive(CellLocation { x: row + MAX_NUMBER_OF_STEPS, y: column + MAX_NUMBER_OF_STEPS, z: MAX_NUMBER_OF_STEPS, w: MAX_NUMBER_OF_STEPS });
                }
            }
        }

        return game;
    }

    pub fn get_alive_count(&self) -> usize {
        (&self.board).iter().filter(|&cell| cell.is_alive).count()
    }
}

