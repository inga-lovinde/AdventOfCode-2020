use enum_map::Enum;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Enum, Eq, PartialEq)]
pub enum State {
    None,
    Floor,
    SeatEmpty,
    SeatOccupied,
}

impl Default for State {
    fn default() -> Self { Self::None }
}

impl State {
    fn from_number(number: u8) -> Self {
        match number {
            0 => Self::None,
            1 => Self::Floor,
            2 => Self::SeatEmpty,
            3 => Self::SeatOccupied,
            _ => panic!("unsupported number {}", number),
        }
    }

    fn get_number(&self) -> u8 {
        match self {
            Self::None => 0, // border should always be 0
            Self::Floor => 1,
            Self::SeatEmpty => 2,
            Self::SeatOccupied => 3,
        }
    }
}

#[derive(Copy, Clone, Enum, EnumIter)]
pub enum Direction {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight,
}

impl Direction {
    fn get_offset(&self) -> u16 {
        match self {
            Self::UpLeft => 0,
            Self::Up => 2,
            Self::UpRight => 4,
            Self::Left => 6,
            Self::Right => 8,
            Self::DownLeft => 10,
            Self::Down => 12,
            Self::DownRight => 14,
        }
    }
}

pub struct CellState {
    neighbours_states: u16,
    state: u8,
}

impl CellState {
    pub fn new() -> Self {
        Self {
            neighbours_states: 0,
            state: 0,
        }
    }

    pub fn update_neighbour_state(&mut self, direction: Direction, new_state: State) -> () {
        self.neighbours_states = (self.neighbours_states & !(0b11 << direction.get_offset())) | ((new_state.get_number() as u16) << direction.get_offset());
    }

    pub fn update_state(&mut self, new_state: State) {
        self.state = new_state.get_number();
    }

    pub fn from_number(number: u32) -> Self {
        CellState {
            state: (number >> 16) as u8,
            neighbours_states: (number & 0xffff) as u16,
        }
    }

    pub fn get_number(&self) -> u32 {
        ((self.state as u32) << 16) | (self.neighbours_states as u32)
    }

    pub fn get_state(&self) -> State {
        State::from_number(self.state)
    }

    pub fn get_neighbour_state(&self, direction: Direction) -> State {
        State::from_number(((self.neighbours_states >> direction.get_offset()) & 0b11) as u8)
    }
}

