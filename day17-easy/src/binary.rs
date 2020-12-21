use enum_map::Enum;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Enum, Eq, PartialEq)]
pub enum State {
    Dead = 0,
    Alive = 1,
}

impl Default for State {
    fn default() -> Self { Self::Dead }
}

impl State {
    fn from_number(number: u8) -> Self {
        match number {
            0 => Self::Dead,
            1 => Self::Alive,
            _ => panic!("unsupported number {}", number),
        }
    }

    fn get_number(&self) -> u8 {
        *self as u8
    }
}

// terribly error-prone but I'm lazy :(
#[derive(Copy, Clone, Enum, EnumIter)]
pub enum Direction {
    MinusMinusMinus = 0,
    MinusMinusSame = 1,
    MinusMinusPlus = 2,
    MinusSameMinus = 3,
    MinusSameSame = 4,
    MinusSamePlus = 5,
    MinusPlusMinus = 6,
    MinusPlusSame = 7,
    MinusPlusPlus = 8,
    SameMinusMinus = 9,
    SameMinusSame = 10,
    SameMinusPlus = 11,
    SameSameMinus = 12,
    SameSamePlus = 13,
    SamePlusMinus = 14,
    SamePlusSame = 15,
    SamePlusPlus = 16,
    PlusMinusMinus = 17,
    PlusMinusSame = 18,
    PlusMinusPlus = 19,
    PlusSameMinus = 20,
    PlusSameSame = 21,
    PlusSamePlus = 22,
    PlusPlusMinus = 23,
    PlusPlusSame = 24,
    PlusPlusPlus = 25,
}

impl Direction {
    fn get_offset(&self) -> u16 {
        *self as u16
    }
}

pub struct CellState {
    neighbours_states: u32,
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
        self.neighbours_states = (self.neighbours_states & !(0b1 << direction.get_offset())) | ((new_state.get_number() as u32) << direction.get_offset());
    }

    pub fn update_state(&mut self, new_state: State) {
        self.state = new_state.get_number();
    }

    pub fn from_number(number: u32) -> Self {
        CellState {
            state: (number >> 26) as u8,
            neighbours_states: (number & 0x3ffffff) as u32,
        }
    }

    pub fn get_number(&self) -> u32 {
        ((self.state as u32) << 26) | (self.neighbours_states as u32)
    }

    pub fn get_state(&self) -> State {
        State::from_number(self.state)
    }

    pub fn get_neighbour_state(&self, direction: Direction) -> State {
        State::from_number(((self.neighbours_states >> direction.get_offset()) & 0b1) as u8)
    }
}

