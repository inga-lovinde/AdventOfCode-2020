use enum_map::EnumMap;
use ndarray::Array2;
use strum::IntoEnumIterator;
use crate::binary::{Direction, State};
use crate::board_metadata::{BoardMetadata,CellLocation};
use crate::rules::Rules;

fn find_neighbour(cell_location: CellLocation, direction: Direction, board_metadata: &BoardMetadata, original_states: &Array2<State>) -> Option<CellLocation> {
    let mut location = cell_location;
    loop {
        match board_metadata.get_neighbour_location(location, direction) {
            Some(new_location) => match original_states[new_location] {
                State::SeatEmpty | State::SeatOccupied => {
                    return Some(new_location);
                },
                _ => {
                    location = new_location;
                },
            },
            None => {
                return None;
            },
        }
    }
}

pub struct RulesHard {}

impl Rules for RulesHard {
    fn get_next_state(current_state: State, neighbour_counts: EnumMap<State, usize>) -> State {
        match current_state {
            State::SeatEmpty => if neighbour_counts[State::SeatOccupied] == 0 { State::SeatOccupied } else { State::SeatEmpty },
            State::SeatOccupied => if neighbour_counts[State::SeatOccupied] >= 5 { State::SeatEmpty } else { State::SeatOccupied },
            other => other
        }
    }

    fn get_neighbours(cell_location: CellLocation, board_metadata: &BoardMetadata, original_states: &Array2<State>) -> EnumMap<Direction, Option<CellLocation>> {
        let mut neighbours = EnumMap::new();

        for direction in Direction::iter() {
            neighbours[direction] = find_neighbour(cell_location, direction, &board_metadata, &original_states)
        }

        neighbours
    }
}
