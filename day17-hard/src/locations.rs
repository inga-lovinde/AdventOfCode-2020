use std::ops::{Add, Index, IndexMut};

use ndarray::Array4;

#[derive(Copy, Clone)]
pub struct CellDiff {
    x: i8,
    y: i8,
    z: i8,
    w: i8,
}

const fn build_minicube_diff_array() -> [CellDiff; 81] {
    let mut result: [CellDiff; 81] = [CellDiff { x: 0, y: 0, z: 0, w: 0 }; 81];
    let mut x = -1;
    let mut y = -1;
    let mut z = -1;
    let mut w = -1;
    loop {
        result[((x*27+y*9+z*3+w) + 40) as usize] = CellDiff {x, y, z, w };

        w += 1;
        if w == 2 {
            w -= 3;
            z += 1;
        }
        if z == 2 {
            z -= 3;
            y += 1;
        }
        if y == 2 {
            y -= 3;
            x += 1;
        }
        if x == 2 {
            break;
        }
    }

    result
}

pub const MINICUBE_DIFF_ARRAY: [CellDiff; 81] = build_minicube_diff_array();

#[derive(Copy, Clone)]
pub struct CellLocation {
    pub x: usize,
    pub y: usize,
    pub z: usize,
    pub w: usize,
}

impl Add<CellDiff> for CellLocation {
    type Output = Self;

    fn add(self, other: CellDiff) -> Self {
        CellLocation {
            x: (self.x as isize + other.x as isize) as usize,
            y: (self.y as isize + other.y as isize) as usize,
            z: (self.z as isize + other.z as isize) as usize,
            w: (self.w as isize + other.w as isize) as usize,
        }
    }
}

impl<T> Index<CellLocation> for Array4<T> {
    type Output = T;
    fn index<'a>(&'a self, cell_location: CellLocation) -> &'a T {
        &self[[cell_location.x, cell_location.y, cell_location.z, cell_location.w]]
    }
}

impl<T> IndexMut<CellLocation> for Array4<T> {
    fn index_mut<'a>(&'a mut self, cell_location: CellLocation) -> &'a mut T {
        &mut self[[cell_location.x, cell_location.y, cell_location.z, cell_location.w]]
    }
}

