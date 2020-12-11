use enum_map::EnumMap;
use ndarray::Array2;
use strum::IntoEnumIterator;
use crate::binary::{Direction, State};
use crate::board_metadata::{BoardMetadata,CellLocation};
use crate::rules::Rules;

pub struct RulesEasy {}

impl Rules for RulesEasy {
    fn get_next_state(current_state: State, neighbour_counts: EnumMap<State, usize>) -> State {
        match current_state {
            State::SeatEmpty => if neighbour_counts[State::SeatOccupied] == 0 { State::SeatOccupied } else { State::SeatEmpty },
            State::SeatOccupied => if neighbour_counts[State::SeatOccupied] >= 4 { State::SeatEmpty } else { State::SeatOccupied },
            other => other
        }
    }

    fn get_neighbours(cell_location: CellLocation, board_metadata: &BoardMetadata, _original_states: &Array2<State>) -> EnumMap<Direction, Option<CellLocation>> {
        let mut neighbours = EnumMap::new();

        for direction in Direction::iter() {
            neighbours[direction] = board_metadata.get_neighbour_location(cell_location, direction);
        }

        neighbours
    }
}
