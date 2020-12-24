use std::ops::{Add, Index, IndexMut};

use ndarray::Array2;

#[derive(Copy, Clone)]
pub struct CellDiff {
    x: i8,
    y: i8,
}

pub const MINICUBE_DIFF_ARRAY: [CellDiff; 6] = [
    CellDiff { x: -1, y:  0 },
    CellDiff { x:  0, y:  1 },
    CellDiff { x:  1, y:  1 },
    CellDiff { x:  1, y:  0 },
    CellDiff { x:  0, y: -1 },
    CellDiff { x: -1, y: -1 },
];

#[derive(Copy, Clone)]
pub struct CellLocation {
    pub x: usize,
    pub y: usize,
}

impl Add<CellDiff> for CellLocation {
    type Output = Self;

    fn add(self, other: CellDiff) -> Self {
        CellLocation {
            x: (self.x as isize + other.x as isize) as usize,
            y: (self.y as isize + other.y as isize) as usize,
        }
    }
}

impl<T> Index<CellLocation> for Array2<T> {
    type Output = T;
    fn index<'a>(&'a self, cell_location: CellLocation) -> &'a T {
        &self[[cell_location.x, cell_location.y]]
    }
}

impl<T> IndexMut<CellLocation> for Array2<T> {
    fn index_mut<'a>(&'a mut self, cell_location: CellLocation) -> &'a mut T {
        &mut self[[cell_location.x, cell_location.y]]
    }
}

