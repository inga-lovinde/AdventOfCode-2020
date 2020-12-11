use std::default::Default;
use std::ops::{Index, IndexMut};
use ndarray::Array2;

use crate::binary::Direction;

#[derive(Copy, Clone)]
pub struct CellLocation {
    row: usize,
    column: usize,
}

impl CellLocation {
    pub fn new(row: usize, column: usize) -> Self {
        Self {
            row,
            column,
        }
    }

    fn new_option(row_option: Option<usize>, column_option: Option<usize>) -> Option<Self> {
        match (row_option, column_option) {
            (Some(row), Some(column)) => Some(Self::new(row, column)),
            _ => None,
        }
    }
}

impl<T> Index<CellLocation> for Array2<T> {
    type Output = T;
    fn index<'a>(&'a self, cell_location: CellLocation) -> &'a T {
        &self[[cell_location.row, cell_location.column]]
    }
}

impl<T> IndexMut<CellLocation> for Array2<T> {
    fn index_mut<'a>(&'a mut self, cell_location: CellLocation) -> &'a mut T {
        &mut self[[cell_location.row, cell_location.column]]
    }
}

pub struct BoardMetadata {
    rows: usize,
    columns: usize,
}

impl BoardMetadata {
    pub fn new(rows: usize, columns: usize) -> Self {
        BoardMetadata {
            rows,
            columns,
        }
    }

    pub fn get_neighbour_location(&self, cell_location: CellLocation, direction: Direction) -> Option<CellLocation> {
        let row = cell_location.row;
        let column = cell_location.column;

        let up = if row > 0 { Some(row-1) } else { None };
        let middle = Some(row);
        let down = if row < self.rows-1 { Some(row+1) } else { None };
        let left = if column > 0 { Some(column-1) } else { None };
        let center = Some(column);
        let right = if column < self.columns-1 { Some(column+1) } else { None };

        match direction {
            Direction::UpLeft => CellLocation::new_option(up, left),
            Direction::Up => CellLocation::new_option(up, center),
            Direction::UpRight => CellLocation::new_option(up, right),
            Direction::Left => CellLocation::new_option(middle, left),
            Direction::Right => CellLocation::new_option(middle, right),
            Direction::DownLeft => CellLocation::new_option(down, left),
            Direction::Down => CellLocation::new_option(down, center),
            Direction::DownRight => CellLocation::new_option(down, right),
        }
    }

    pub fn create_board_default<T: Default>(&self) -> Array2<T> {
        Array2::default((self.rows, self.columns))
    }

    pub fn create_board_from_shape_fn<T, F: Fn(CellLocation) -> T>(&self, f: F) -> Array2<T> {
        Array2::from_shape_fn((self.rows, self.columns), |(row, column)| f(CellLocation::new(row, column)))
    }
}
