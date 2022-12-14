use core::fmt;
use std::fmt::{Debug, Display, Formatter};

pub trait Coord {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
    fn coords(&self) -> (usize, usize) {
        (self.x(), self.y())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPoint {
    x: usize,
    y: usize,
}

impl GridPoint {
    pub fn new(x: usize, y: usize) -> Self {
        GridPoint { x, y }
    }

    pub fn add(&self, (x, y): (isize, isize)) -> Self {
        Self {
            x: (self.x as isize + x) as usize,
            y: (self.y as isize + y) as usize,
        }
    }
}

impl Coord for GridPoint {
    fn x(&self) -> usize {
        self.x
    }

    fn y(&self) -> usize {
        self.y
    }
}

impl Coord for (usize, usize) {
    fn x(&self) -> usize {
        self.0
    }

    fn y(&self) -> usize {
        self.1
    }

    fn coords(&self) -> (usize, usize) {
        *self
    }
}

#[allow(dead_code)]
const DIRS: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub const ORTHOGONAL_DIRS: [(isize, isize); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

#[derive(Debug, Clone, PartialEq)]
pub struct Grid<T> {
    cells: Vec<T>,
    pub w: usize,
    pub h: usize,
}

impl<T: Clone + Copy + PartialEq + Debug> Grid<T> {
    pub fn new(cells: Vec<T>, h: usize, w: usize) -> Self {
        Self { cells, w, h }
    }

    pub fn get(&self, c: &impl Coord) -> Option<&T> {
        if c.x() < self.w && c.y() < self.h {
            self.cells.get(c.x() + c.y() * self.w)
        } else {
            None
        }
    }

    pub fn set(&mut self, c: &impl Coord, v: T) {
        if let Some(e) = self.cells.get_mut(c.x() + c.y() * self.w) {
            *e = v;
        }
    }

    pub fn filter(&self, needle: T) -> Vec<GridPoint> {
        self.cells
            .iter()
            .cloned()
            .enumerate()
            .filter_map(|(i, v)| {
                (needle == v).then_some(GridPoint::new(
                    i % self.w,
                    (i as f64 / self.w as f64).floor() as usize,
                ))
            })
            .collect()
    }
}

impl<T: fmt::Display> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        for row in self.cells.chunks(self.w) {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
