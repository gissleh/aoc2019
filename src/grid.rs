use std::cmp::Ordering;
use std::f64::consts::PI;
use num::abs;
use crate::num::Integer;

#[derive(Clone)]
pub struct BigGrid<T> {
    chunks: Vec<[T; 1024]>,
    map: Grid<usize>,
    default_value: T,
    tl_x: isize,
    tl_y: isize,
    br_x: isize,
    br_y: isize,
}

impl<T> BigGrid<T> where T: Clone + Copy + std::fmt::Debug {
    fn chunk_index(&self, x: isize, y: isize) -> (usize, isize, isize, usize) {
        let chunk_x = if x < 0 { (x / 32) - 1 } else { x / 32 };
        let chunk_y = if y < 0 { (y / 32) - 1 } else { y / 32 };

        let chunk_index = self.map.get(chunk_x, chunk_y);
        let relative_x = x - (chunk_x * 32);
        let relative_y = y - (chunk_y * 32);
        let relative_index = (relative_y * 32 + relative_x) as usize;

        (chunk_index, chunk_x, chunk_y, relative_index)
    }

    pub fn get(&self, x: isize, y: isize) -> T {
        let (chunk_index, _, _, relative_index) = self.chunk_index(x, y);
        if chunk_index == 0 {
            return self.default_value;
        }

        let chunk = &self.chunks[chunk_index - 1];

        chunk[relative_index]
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> &mut T {
        let (chunk_index, chunk_x, chunk_y, relative_index) = self.chunk_index(x, y);

        let chunk = if chunk_index > 0 {
            &mut self.chunks[chunk_index - 1]
        } else {
            self.map.set(chunk_x, chunk_y, self.chunks.len());
            self.chunks.push([self.default_value; 1024]);

            self.chunks.last_mut().unwrap()
        };

        if x < self.tl_x {
            self.tl_x = x;
        } else if x >= self.br_x {
            self.br_x = x;
        }
        if y < self.tl_y {
            self.tl_y = y;
        } else if y >= self.br_y {
            self.br_y = y;
        }

        &mut chunk[relative_index]
    }

    pub fn bounds_tl(&self) -> (isize, isize) {
        (self.tl_x, self.tl_y)
    }

    pub fn bounds_br(&self) -> (isize, isize) {
        (self.br_x, self.br_y)
    }

    pub fn to_exact(&self) -> Grid<T> {
        let offset_x = -self.tl_x;
        let offset_y = -self.tl_y;
        let width = ((self.br_x - self.tl_x) + 1) as usize;
        let height = ((self.br_y - self.tl_y) + 1) as usize;

        let mut grid = Grid::new(width, height, offset_x, offset_y, self.default_value);

        for x in self.tl_x..self.br_x {
            for y in self.tl_y..self.br_y {
                grid.set(x, y, self.get(x, y));
            }
        }

        grid
    }

    pub fn new(max_width: usize, max_height: usize, default_value: T) -> BigGrid<T> {
        BigGrid {
            chunks: Vec::with_capacity(128),
            map: Grid::new(
                max_width / 32, max_height / 32,
                (max_width / 64) as isize, (max_height / 64) as isize,
                0,
            ),
            default_value,
            tl_x: 0,
            tl_y: 0,
            br_x: 0,
            br_y: 0,
        }
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    default_value: T,
    width: usize,
    height: usize,
    offset_x: isize,
    offset_y: isize,
}

impl<T> Grid<T> {
    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}

impl<T> Grid<T> where T: Clone + Copy + std::fmt::Debug {
    fn index(&self, x: isize, y: isize) -> usize {
        ((y + self.offset_y) as usize * self.width) + (x + self.offset_x) as usize
    }

    pub fn set_data(&mut self, data: &[T]) {
        if data.len() != self.data.len() {
            panic!("Data length mismatch");
        }

        self.data.copy_from_slice(data);
    }

    pub fn clear(&mut self) {
        let len = self.data.len();
        self.data.truncate(0);
        self.data.resize(len, self.default_value);
    }

    pub fn get(&self, x: isize, y: isize) -> T {
        self.data[self.index(x, y)]
    }

    pub fn get_oob(&self, x: isize, y: isize) -> T {
        let x = x + self.offset_x;
        let y = y + self.offset_y;
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return self.default_value;
        }

        self.data[self.index(x, y)]
    }

    pub fn get_mut(&mut self, x: isize, y: isize) -> &mut T {
        let index = self.index(x, y);

        &mut self.data[index]
    }

    pub fn set(&mut self, x: isize, y: isize, v: T) {
        let index = self.index(x, y);
        self.data[index] = v;
    }

    pub fn new(width: usize, height: usize, offset_x: isize, offset_y: isize, default_value: T) -> Grid<T> {
        Grid{
            width, height, offset_x, offset_y, default_value,
            data: vec![default_value; width * height],
        }
    }

    pub fn empty(default_value: T) -> Grid<T> {
        Grid{
            width: 0,
            height: 0,
            offset_x: 0,
            offset_y: 0,
            default_value,
            data: Vec::new(),
        }
    }
}

pub fn grid_direction(x1: isize, y1: isize, x2: isize, y2: isize) -> (isize, isize) {
    let (dx, dy, _) = grid_direction_len(x1, y1, x2, y2);

    (dx, dy)
}

pub fn grid_direction_len(x1: isize, y1: isize, x2: isize, y2: isize) -> (isize, isize, isize) {
    let diff_x = x2 - x1;
    let diff_y = y2 - y1;
    let gcd = abs(diff_x).gcd(&diff_y);

    if gcd == 0 {
        return (0, 0, 0);
    }

    (diff_x / gcd, diff_y / gcd, gcd)
}

pub fn direction_atan2(dx: isize, dy: isize) -> f64 {
    let atan2 = 90.0 + (dy as f64).atan2(dx as f64) * (180.0 / PI);

    if atan2 < 0.0 {
        atan2 + 360.0
    } else {
        atan2
    }
}

pub fn cmp_f64(a: f64, b: f64) -> Ordering {
    let bi = (b * 1000.0) as i32;

    ((a * 1000.0) as i32).cmp(&bi)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_indexes() {
        let mut grid = Grid::new(8, 8, 4, 4, 0);
        grid.set(-4, -4, 16);
        grid.set(-4, -3, 14);
        grid.set(-1, 3, 17);

        assert_eq!(grid.get(-4, -4), 16);
        assert_eq!(grid.get(-4, -3), 14);
        assert_eq!(grid.get(-1, 3), 17);
        assert_eq!(grid.get(3, 3), 0);

        grid.clear();

        assert_eq!(grid.get(-4, -4), 0);
        assert_eq!(grid.get(-4, -3), 0);
        assert_eq!(grid.get(-1, 3), 0);
        assert_eq!(grid.get(3, 3), 0);
    }

    #[test]
    fn test_grid_direction() {
        assert_eq!(grid_direction(0, 0, 6, 3), (2, 1));
        assert_eq!(grid_direction(0, 0, 12, 6), (2, 1));
        assert_eq!(grid_direction(0, 0, 1, 5), (1, 5));
        assert_eq!(grid_direction(0, 0, -1, -5), (-1, -5));
    }

    #[test]
    fn test_grid_direction_atan2() {
        assert_eq!(direction_atan2(0, -1), 0.0);
        assert_eq!(direction_atan2(1, 0), 90.0);
        assert_eq!(direction_atan2(0, 1), 180.0);
        assert_eq!(direction_atan2(-1, 0), 270.0);
    }
}