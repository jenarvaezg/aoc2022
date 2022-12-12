use std::collections::{HashSet, VecDeque};

use crate::grid::{Coord, Grid, GridPoint, ORTHOGONAL_DIRS};
use crate::solver::Solver;

fn shortest_route(grid: &Grid<char>, start: GridPoint, end: GridPoint) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    while let Some((point, cost)) = queue.pop_front() {
        if point == end {
            return Some(cost);
        }
        let current_value = grid.get(&point).unwrap();
        for dir in ORTHOGONAL_DIRS {
            let candidate = GridPoint::new(
                (point.x() as isize + dir.0) as usize,
                (point.y() as isize + dir.1) as usize,
            );
            let candidate_value = grid.get(&candidate);
            if candidate_value.is_some()
                && (*current_value as usize + 1) >= *candidate_value.unwrap() as usize
                && !visited.contains(&candidate)
            {
                visited.insert(candidate);
                queue.push_back((candidate, cost + 1));
            }
        }
    }
    None
}

pub struct Problem;

impl Solver for Problem {
    type Input = (Grid<char>, GridPoint, GridPoint);
    type Output = usize;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        let lines: Vec<&str> = raw_input.lines().collect();
        let h = lines.len();
        let w = lines[0].len();
        let mut start = GridPoint::new(0, 0);
        let mut end = GridPoint::new(0, 0);
        let cells: Vec<_> = lines
            .iter()
            .flat_map(|s| s.chars())
            .enumerate()
            .map(|(i, c)| {
                match c {
                    'S' => {
                        if i != 0 {
                            //ugly
                            start = GridPoint::new(0, (i as f64 / w as f64).ceil() as usize);
                        }
                        'a'
                    }
                    'E' => {
                        //ugly
                        end = GridPoint::new(i % w, (i as f64 / w as f64).floor() as usize);
                        'z'
                    }
                    c => c,
                }
            })
            .collect();

        Some((Grid::new(cells, h, w), start, end))
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(shortest_route(&input.0, input.1, input.2).unwrap())
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        let starting_points = input.0.find('a');
        Some(
            starting_points
                .into_iter()
                .filter_map(|start| shortest_route(&input.0, start, input.2))
                .min()
                .unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    #[test]
    fn test_first() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_second() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, Some(29));
    }
}
