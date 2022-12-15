use crate::{
    grid::{Coord, Grid, GridPoint},
    solver::Solver,
};

use std::ops::Range;

pub struct Problem;

impl Solver for Problem {
    type Input = Grid<char>;
    type Output = usize;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        let mut grid = Grid::new(vec!['.'; 200 * 700], 200, 700);
        let lines: Vec<Vec<GridPoint>> = raw_input
            .lines()
            .map(|l| {
                l.split("->")
                    .map(|coord| {
                        let (x, y) = coord.trim().split_once(",").unwrap();
                        GridPoint::new(x.parse().unwrap(), y.parse().unwrap())
                    })
                    .collect()
            })
            .collect();

        let mut lowest = 0;
        for line in lines.into_iter() {
            for window in line.windows(2) {
                let (start, target) = (window[0], window[1]);
                lowest = lowest.max(start.y());
                let mut pos = start.clone();
                grid.set(&pos, '#');

                while pos.x() != target.x() || pos.y() != target.y() {
                    pos = pos.add((
                        (target.x() as isize - pos.x() as isize).signum(),
                        (target.y() as isize - pos.y() as isize).signum(),
                    ));
                    grid.set(&pos, '#');
                }
            }
        }

        Some(grid)
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut grid = input.clone();
        let lowest = grid
            .filter('#')
            .iter()
            .max_by(|x, y| x.y().cmp(&y.y()))
            .unwrap()
            .y();

        'outer: loop {
            // new sand
            let mut sand = GridPoint::new(500, 0);

            loop {
                if sand.y() + 1 > lowest {
                    break 'outer;
                }
                //down
                if *grid.get(&sand.add((0, 1)))? == '.' {
                    sand = sand.add((0, 1));
                    continue;
                }

                //left
                if *grid.get(&sand.add((-1, 1)))? == '.' {
                    sand = sand.add((-1, 1));
                    continue;
                }
                //right
                if *grid.get(&sand.add((1, 1)))? == '.' {
                    sand = sand.add((1, 1));
                    continue;
                }

                grid.set(&sand, 'o');
                break;
            }
        }

        Some(grid.filter('o').len())
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut grid = input.clone();
        let lowest = grid
            .filter('#')
            .iter()
            .max_by(|x, y| x.y().cmp(&y.y()))
            .unwrap()
            .y();

        for x in 0..1000 {
            grid.set(&(x as usize, (lowest + 2) as usize), '#');
        }

        loop {
            // new sand
            let mut sand = GridPoint::new(500, 0);

            if *grid.get(&sand)? == 'o' {
                break;
            }

            loop {
                //down
                if *grid.get(&sand.add((0, 1)))? == '.' {
                    sand = sand.add((0, 1));
                    continue;
                }

                //left
                if *grid.get(&sand.add((-1, 1)))? == '.' {
                    sand = sand.add((-1, 1));
                    continue;
                }
                //right
                if *grid.get(&sand.add((1, 1)))? == '.' {
                    sand = sand.add((1, 1));
                    continue;
                }

                grid.set(&sand, 'o');
                break;
            }
        }

        Some(grid.filter('o').len())
    }
}

#[allow(dead_code)]
fn print_grid(grid: &Grid<char>, xs: Range<usize>, ys: Range<usize>) {
    for y in ys {
        for x in xs.clone() {
            let v = grid.get(&(x, y)).unwrap();
            print!("{v}");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9"#;

    #[test]
    fn test_first() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(24));
    }

    #[test]
    fn test_second() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, Some(93));
    }
}
