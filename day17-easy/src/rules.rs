use enum_map::EnumMap;
use ndarray::Array2;
use crate::binary::{Direction, State};
use crate::board_metadata::{BoardMetadata,CellLocation};

pub trait Rules {
    fn get_next_state(current_state: State, neighbour_counts: EnumMap<State, usize>) -> State;
    fn get_neighbours(cell_location: CellLocation, board_metadata: &BoardMetadata, original_states: &Array2<State>) -> EnumMap<Direction, Option<CellLocation>>;
}
