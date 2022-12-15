use core::fmt;
use std::fmt::{Debug, Display, Formatter};

pub trait Coord {
    fn x(&self) -> isize;
    fn y(&self) -> isize;

    fn distance(&self, other: &Self) -> usize {
        self.x().abs_diff(other.x()) + self.y().abs_diff(other.y())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridPoint {
    x: isize,
    y: isize,
}

impl GridPoint {
    pub fn new(x: isize, y: isize) -> Self {
        GridPoint { x, y }
    }

    pub fn add(&self, (x, y): (isize, isize)) -> Self {
        Self {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

impl Coord for GridPoint {
    fn x(&self) -> isize {
        self.x
    }

    fn y(&self) -> isize {
        self.y
    }
}

impl Coord for (usize, usize) {
    fn x(&self) -> isize {
        self.0 as isize
    }

    fn y(&self) -> isize {
        self.1 as isize
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
        if c.x() < self.w as isize && c.y() < self.h as isize {
            self.cells.get((c.x() + c.y() * self.w as isize) as usize)
        } else {
            None
        }
    }

    pub fn set(&mut self, c: &impl Coord, v: T) {
        if let Some(e) = self
            .cells
            .get_mut((c.x() + c.y() * self.w as isize) as usize)
        {
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
                    (i % self.w) as isize,
                    (i as f64 / self.w as f64).floor() as isize,
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
