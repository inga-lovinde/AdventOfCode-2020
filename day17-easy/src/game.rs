use enum_map::EnumMap;
use ndarray::{Array2, Array3};
use strum::IntoEnumIterator;

use crate::binary::CellState;
use crate::binary::Direction;
use crate::binary::State;
use crate::board_metadata::BoardMetadata;
use crate::board_metadata::CellLocation;
use crate::rules::Rules;

const RESERVE: usize = 10;

struct CellInfo {
    neighbours: EnumMap<Direction, Option<CellLocation>>,
    state: CellState,
}

impl CellInfo {
    fn new(neighbours: EnumMap<Direction, Option<CellLocation>>) -> CellInfo {
        CellInfo {
            neighbours,
            state: CellState::new(),
        }
    }

    fn update_neighbour_state(&mut self, direction: Direction, new_state: State) -> () {
        self.state.update_neighbour_state(direction, new_state)
    }

    fn update_state(&mut self, new_state: State) {
        self.state.update_state(new_state)
    }
}

pub struct Game {
    cell_rules: Vec<State>,
    board: Array3<CellInfo>,
    corner: CellLocation,
}

impl Game {
    // only updates the state of this cell for it and its neighbours;
    // only state of this cell is used from new_cell
    fn update_cell(&mut self, location: CellLocation, new_state: State) {
        //println!("Updating cell {}:{}", location.row, location.column);
        self.board[location].update_state(new_state);

        for direction in Direction::iter() {
            match self.board[location].neighbours[direction] {
                Some(neighbour_location) => {
                    //println!("Updating neighbour cell {}:{}", neighbour_location.row, neighbour_location.column);
                    self.board[neighbour_location].update_neighbour_state(direction, new_state);
                },
                _ => {},
            }
        }
    }

    fn build_cell_rules<T: Rules>() -> Vec<State> {
        let mut result = vec![State::Dead; 1 << 27];

        for i in 0..1usize << 27 {
            let original_state = CellState::from_number(i as u32);
            let mut neighbour_counts = EnumMap::new();
            for direction in Direction::iter() {
                neighbour_counts[original_state.get_neighbour_state(direction)] += 1usize;
            }

            let new_state = T::get_next_state(original_state.get_state(), neighbour_counts);
            result[i] = new_state;
            //println!("Rule #{}: for state_counts [{}, {}, {}, {}] and old state {} new state is {}", i, state_counts[0], state_counts[1], state_counts[2], state_counts[3], current_state, new_state);
        }

        result
    }

    fn get_next_state(&self, cell_info: &CellInfo) -> State {
        self.cell_rules[cell_info.state.get_number() as usize]
    }

    pub fn next_step(&mut self) -> usize {
        let mut changes: Vec<_> = vec![];
        for x in 0..self.corner.x {
            for y in 0..self.corner.y {
                for z in 0..self.corner.z {
                    let location = CellLocation::new(x, y, z);
                    let cell = &self.board[location];
                    let next_state = self.get_next_state(cell);
                    //println!("location: {}:{}, neighbours state {}, old state {}, next state {}", location.row, location.column, cell.neighbours_states, cell.state, next_state);
                    if next_state != cell.state.get_state() {
                        changes.push((location, next_state));
                    }
                }
            }
        }

        let changes_count = changes.len();
        for (location, new_state) in changes {
            self.update_cell(location, new_state);
        }

        changes_count
    }

    pub fn from_input<R: Rules>(input_data: &[String]) -> Self {
        let rows = input_data.len();
        let columns = input_data[0].len();
        let mut states = Array2::default((rows, columns));
        
        for row in 0..rows {
            let chars = input_data[row].chars().collect::<Vec<_>>();
            for column in 0..columns {
                let ch = chars[column];
                states[[row, column]] = match ch {
                    '.' => State::Dead,
                    '#' => State::Alive,
                    _ => panic!("unsupported state"),
                }
            }
        }
        
        let corner = CellLocation::new(rows + 2*RESERVE, columns + 2*RESERVE, 1 + 2*RESERVE);
        let board_metadata = BoardMetadata::new(corner);
        let board = board_metadata.create_board_from_shape_fn(|cell_location| {
            CellInfo::new(R::get_neighbours(cell_location, &board_metadata, &states))
        });

        let cell_rules = Self::build_cell_rules::<R>();

        let mut game = Game {
            board,
            cell_rules,
            corner,
        };

        for row in 0..rows {
            for column in 0..columns {
                let location = CellLocation::new(row + RESERVE, column + RESERVE, RESERVE);
                game.update_cell(location, states[(row, column)]);
            }
        }

        return game;
    }

    /*pub fn print_board(&self) {
        for row in (&self.board).genrows() {
            println!("{}", row.iter().map(|cell| {
                match cell.state.get_state() {
                    State::Floor => '.',
                    State::SeatEmpty => 'L',
                    State::SeatOccupied => '#',
                    State::None => '0',
                }
            }).collect::<String>());
        }
    }*/

    pub fn get_count_of_cells_for_state(&self, state: State) -> usize {
        (&self.board).iter().filter(|&cell| cell.state.get_state() == state).count()
    }
}
