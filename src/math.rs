use num::{Integer, abs};
use std::f64::consts::PI;

pub struct Permutations<T> where T : Clone + Copy + std::fmt::Debug {
    data: Vec<T>,
    stack: Vec<usize>,
    popped: bool,
}

impl<T> Permutations<T> where T : Clone + Copy + std::fmt::Debug {
    pub fn count(&mut self) -> usize {
        let mut count = 0;

        while let Some(_) = self.next() {
            count += 1;
        }

        count
    }

    pub fn next(&mut self) -> Option<&[T]> {
        if self.stack.len() == 0 {
            return None
        }

        let from = self.stack.len() - 1;
        let to = self.data.len();
        let mut i = *self.stack.last().unwrap();

        if self.popped {
            self.data.swap(from, i);
            *self.stack.last_mut().unwrap() += 1;
            i += 1;
        }
        self.popped = false;

        if self.stack.len() == self.data.len() {
            self.stack.pop();
            self.popped = true;

            Some(&self.data)
        } else {
            if i == to {
                self.stack.pop();
                self.popped = true;

                self.next()
            } else {
                self.data.swap(from, i);
                self.stack.push(from + 1);

                self.next()
            }
        }
    }

    pub fn new(items: &[T]) -> Permutations<T> where T: Clone + Copy + std::fmt::Debug {
        if items.len() < 2 {
            panic!("permutations of too short list.")
        }

        let mut stack = Vec::with_capacity(items.len());
        stack.push(0);

        Permutations{
            data: Vec::from(items),
            popped: false,
            stack,
        }
    }
}

pub struct GridDirectionIterator {
    index: isize,
    x: isize,
    y: isize,
    width: isize,
    height: isize,
    prev: (isize, isize),
}

impl Iterator for GridDirectionIterator {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == ((2 * self.width) + (2 * self.height)) - 3 {
            None
        } else {
            let w = self.width;
            let h = self.height;
            let wh = w + h - 1;
            let whw = wh + w - 1;
            let whwh = whw + h - 1;

            let (x2, y2) = if self.index < w {
                (self.index, 0)
            } else if self.index < wh {
                (w - 1, self.index - (w) + 1)
            } else if self.index < whw {
                (whw - self.index, h - 1)
            } else {
                (0, whwh - self.index)
            };

            self.index += 1;

            let (dx, dy) = grid_direction(self.x, self.y, x2, y2);
            if self.prev == (dx, dy) {
                self.next()
            } else {
                self.prev = (dx, dy);
                Some((dx, dy))
            }
        }
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    data: Vec<T>,
    default_value: T,
    width: usize,
    offset_x: isize,
    offset_y: isize,
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
        return self.data[self.index(x, y)]
    }

    pub fn set(&mut self, x: isize, y: isize, v: T) {
        let index = self.index(x, y);
        self.data[index] = v;
    }

    pub fn directions(&self, x: isize, y: isize) -> GridDirectionIterator {
        let height = (self.data.len() / self.width) as isize;

        return GridDirectionIterator{
            index: 0,
            width: (3 * self.width) as isize,
            height: (3 * height),
            x: x + self.offset_x + (self.width as isize),
            y: y + self.offset_y + height,
            prev: (0, 0),
        }
    }

    pub fn new(width: usize, height: usize, offset_x: isize, offset_y: isize, default_value: T) -> Grid<T> {
        Grid{
            width, offset_x, offset_y, default_value,
            data: vec![default_value; width * height],
        }
    }
}

pub fn grid_direction(x1: isize, y1: isize, x2: isize, y2: isize) -> (isize, isize) {
    let diff_x = x2 - x1;
    let diff_y = y2 - y1;
    let gcd = abs(diff_x).gcd(&diff_y);

    if gcd == 0 {
        return (0, 0);
    }

    (diff_x / gcd, diff_y / gcd)
}

pub fn direction_magnitude(x1: isize, y1: isize, x2: isize, y2: isize) -> (isize, isize) {
    let diff_x = x2 - x1;
    let diff_y = y2 - y1;

    let mut atan2 = 90.0 + (diff_y as f64).atan2(diff_x as f64) * (180.0 / PI);
    if atan2 < 0.0 {
        atan2 += 360.0;
    }

    ((atan2 * 1000.0) as isize, abs(diff_x) + abs(diff_y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_permutations_count() {
        assert_eq!(Permutations::new(&[1, 2, 3]).count(), 6);
        assert_eq!(Permutations::new(&[1, 2, 3, 4]).count(), 24);
        assert_eq!(Permutations::new(&[1, 2, 3, 4, 5]).count(), 120);
        assert_eq!(Permutations::new(&[1, 2, 3, 4, 5, 6]).count(), 720);
    }

    #[test]
    fn test_permutations_all() {
        let mut hs: HashSet<Vec<i32>> = HashSet::with_capacity(320);
        let mut perm = Permutations::new(&[1, 2, 3]);

        while let Some(v) = perm.next() {
            println!("{:?}", v);

            if hs.contains(v) {
                panic!("repeated permutation: {:?}", v);
            }
            hs.insert(v.to_vec());
        }
    }

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
    fn test_grid_iterator_directions() {
        let mut grid = Grid::new(5, 5, 2, 2, 0);
        let directions: Vec<(isize, isize)> = grid.directions(0, 0).collect();

        for (dx, dy) in directions.iter().cloned() {
            println!("{}, {}", dx, dy);
        }

        assert_eq!(directions.len(), 8);
    }
}