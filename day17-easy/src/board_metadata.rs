use std::default::Default;
use std::ops::{Index, IndexMut};
use ndarray::Array3;

use crate::binary::Direction;

#[derive(Copy, Clone)]
pub struct CellLocation {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl CellLocation {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    fn new_option(x_option: Option<usize>, y_option: Option<usize>, z_option: Option<usize>) -> Option<Self> {
        match (x_option, y_option, z_option) {
            (Some(x), Some(y), Some(z)) => Some(Self::new(x, y, z)),
            _ => None,
        }
    }
}

impl<T> Index<CellLocation> for Array3<T> {
    type Output = T;
    fn index<'a>(&'a self, cell_location: CellLocation) -> &'a T {
        &self[[cell_location.x, cell_location.y, cell_location.z]]
    }
}

impl<T> IndexMut<CellLocation> for Array3<T> {
    fn index_mut<'a>(&'a mut self, cell_location: CellLocation) -> &'a mut T {
        &mut self[[cell_location.x, cell_location.y, cell_location.z]]
    }
}

pub struct BoardMetadata {
    corner: CellLocation,
}

struct CellLocationWithBoardMetadata {
    corner: CellLocation,
    location: Option<CellLocation>,
}

impl CellLocationWithBoardMetadata {
    fn new(corner: CellLocation, location: Option<CellLocation>) -> CellLocationWithBoardMetadata {
        CellLocationWithBoardMetadata {
            corner,
            location,
        }
    }

    fn get_x_minus(&self) -> CellLocationWithBoardMetadata {
        CellLocationWithBoardMetadata::new(self.corner, match self.location {
            Some(location) if location.x > 0 => Some(CellLocation { x: location.x-1, ..location }),
            _ => None
        })
    }

    fn get_x_plus(&self) -> CellLocationWithBoardMetadata {
        CellLocationWithBoardMetadata::new(self.corner, match self.location {
            Some(location) if location.x < self.corner.x - 1 => Some(CellLocation { x: location.x+1, ..location }),
            _ => None
        })
    }

    fn get_y_minus(&self) -> CellLocationWithBoardMetadata {
        CellLocationWithBoardMetadata::new(self.corner, match self.location {
            Some(location) if location.y > 0 => Some(CellLocation { y: location.y-1, ..location }),
            _ => None
        })
    }

    fn get_y_plus(&self) -> CellLocationWithBoardMetadata {
        CellLocationWithBoardMetadata::new(self.corner, match self.location {
            Some(location) if location.y < self.corner.y - 1 => Some(CellLocation { y: location.y+1, ..location }),
            _ => None
        })
    }

    fn get_z_minus(&self) -> CellLocationWithBoardMetadata {
        CellLocationWithBoardMetadata::new(self.corner, match self.location {
            Some(location) if location.z > 0 => Some(CellLocation { z: location.z-1, ..location }),
            _ => None
        })
    }

    fn get_z_plus(&self) -> CellLocationWithBoardMetadata {
        CellLocationWithBoardMetadata::new(self.corner, match self.location {
            Some(location) if location.z < self.corner.z - 1 => Some(CellLocation { z: location.z+1, ..location }),
            _ => None
        })
    }
}

impl BoardMetadata {
    pub fn new(corner: CellLocation) -> Self {
        BoardMetadata {
            corner,
        }
    }

    pub fn get_neighbour_location(&self, cell_location: CellLocation, direction: Direction) -> Option<CellLocation> {
        let cell_with_metadata = CellLocationWithBoardMetadata::new(self.corner, Some(cell_location));

        // terribly error-prone but I'm lazy :(
        match direction {
            Direction::MinusMinusMinus => cell_with_metadata.get_x_minus().get_y_minus().get_z_minus().location,
            Direction::MinusMinusSame  => cell_with_metadata.get_x_minus().get_y_minus().location,
            Direction::MinusMinusPlus  => cell_with_metadata.get_x_minus().get_y_minus().get_z_plus().location,
            Direction::MinusSameMinus  => cell_with_metadata.get_x_minus().get_z_minus().location,
            Direction::MinusSameSame   => cell_with_metadata.get_x_minus().location,
            Direction::MinusSamePlus   => cell_with_metadata.get_x_minus().get_z_plus().location,
            Direction::MinusPlusMinus  => cell_with_metadata.get_x_minus().get_y_plus().get_z_minus().location,
            Direction::MinusPlusSame   => cell_with_metadata.get_x_minus().get_y_plus().location,
            Direction::MinusPlusPlus   => cell_with_metadata.get_x_minus().get_y_plus().get_z_plus().location,
            Direction::SameMinusMinus => cell_with_metadata.get_y_minus().get_z_minus().location,
            Direction::SameMinusSame  => cell_with_metadata.get_y_minus().location,
            Direction::SameMinusPlus  => cell_with_metadata.get_y_minus().get_z_plus().location,
            Direction::SameSameMinus  => cell_with_metadata.get_z_minus().location,
            Direction::SameSamePlus   => cell_with_metadata.get_z_plus().location,
            Direction::SamePlusMinus  => cell_with_metadata.get_y_plus().get_z_minus().location,
            Direction::SamePlusSame   => cell_with_metadata.get_y_plus().location,
            Direction::SamePlusPlus   => cell_with_metadata.get_y_plus().get_z_plus().location,
            Direction::PlusMinusMinus => cell_with_metadata.get_x_plus().get_y_minus().get_z_minus().location,
            Direction::PlusMinusSame  => cell_with_metadata.get_x_plus().get_y_minus().location,
            Direction::PlusMinusPlus  => cell_with_metadata.get_x_plus().get_y_minus().get_z_plus().location,
            Direction::PlusSameMinus  => cell_with_metadata.get_x_plus().get_z_minus().location,
            Direction::PlusSameSame   => cell_with_metadata.get_x_plus().location,
            Direction::PlusSamePlus   => cell_with_metadata.get_x_plus().get_z_plus().location,
            Direction::PlusPlusMinus  => cell_with_metadata.get_x_plus().get_y_plus().get_z_minus().location,
            Direction::PlusPlusSame   => cell_with_metadata.get_x_plus().get_y_plus().location,
            Direction::PlusPlusPlus   => cell_with_metadata.get_x_plus().get_y_plus().get_z_plus().location,
        }
    }

    pub fn create_board_from_shape_fn<T, F: Fn(CellLocation) -> T>(&self, f: F) -> Array3<T> {
        Array3::from_shape_fn((self.corner.x, self.corner.y, self.corner.z), |(x, y, z)| f(CellLocation::new(x, y, z)))
    }
}
