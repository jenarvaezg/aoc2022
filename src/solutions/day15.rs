use std::collections::HashSet;

use itertools::zip;

use crate::{grid::Coord, grid::GridPoint, solver::Solver};

pub struct Problem;

fn get_gridpoint(s: &str) -> GridPoint {
    let (raw_x, raw_y) = s.split_once(", ").unwrap();
    let y = raw_y[2..].parse().unwrap();
    let x = raw_x.split_once("x=").unwrap().1.parse().unwrap();

    GridPoint::new(x, y)
}

fn impossibles_at_row(
    starts: Vec<GridPoint>,
    distances: Vec<usize>,
    row: usize,
) -> HashSet<GridPoint> {
    let mut impossible_positions: HashSet<GridPoint> = HashSet::new();

    for (start, distance) in zip(starts, distances) {
        for x in -6000000..6000000 {
            let point = GridPoint::new(x, row as isize);
            if start.distance(&point) <= distance {
                impossible_positions.insert(point);
            }
        }
    }

    impossible_positions
}

fn blind_spot(sensors_with_distances: &Vec<(GridPoint, usize)>, point: &GridPoint) -> bool {
    sensors_with_distances
        .iter()
        .all(|&(beacon, distance)| beacon.distance(point) > distance)
}

impl Problem {
    fn _solve_first(&self, input: &Vec<(GridPoint, GridPoint)>, row: usize) -> Option<usize> {
        let distances: Vec<usize> = input
            .iter()
            .map(|(sensor, beacon)| sensor.distance(beacon))
            .collect();
        let sensors = input.iter().map(|(sensor, _)| *sensor).collect();
        let mut impossible_positions = impossibles_at_row(sensors, distances, row);

        for (_, beacon) in input.iter() {
            impossible_positions.remove(beacon);
        }

        Some(impossible_positions.len())
    }

    fn _solve_second(&self, input: &Vec<(GridPoint, GridPoint)>, at_most: isize) -> Option<usize> {
        let distances: Vec<usize> = input
            .iter()
            .map(|(sensor, beacon)| sensor.distance(beacon))
            .collect();
        let sensors: Vec<_> = input.iter().map(|(sensor, _)| *sensor).collect();
        let sensors_with_distances: Vec<(GridPoint, usize)> = zip(sensors, distances).collect();

        sensors_with_distances
            .iter()
            .filter_map(|&(sensor, distance)| {
                let (x, y) = (sensor.x(), sensor.y());
                let mut d_y = 0;
                for n_x in x - distance as isize - 1..x.min(at_most) {
                    if n_x < 0 {
                        d_y += 1;
                        continue;
                    }

                    let point = GridPoint::new(n_x, y + d_y);
                    if point.y() <= at_most && blind_spot(&sensors_with_distances, &point) {
                        return Some(point);
                    }

                    let point = GridPoint::new(n_x, y - d_y);
                    if point.y() >= 0 && blind_spot(&sensors_with_distances, &point) {
                        return Some(point);
                    }

                    d_y += 1;
                }
                None
            })
            .map(|p| (p.x() * 4000000 + p.y()) as usize)
            .next()
    }
}

impl Solver for Problem {
    type Input = Vec<(GridPoint, GridPoint)>;
    type Output = usize;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        Some(
            raw_input
                .lines()
                .filter_map(|l| {
                    let (raw_sensor, raw_beacon) = l.split_once(": ")?;

                    Some((get_gridpoint(raw_sensor), get_gridpoint(raw_beacon)))
                })
                .collect(),
        )
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        self._solve_first(&input, 2000000)
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        self._solve_second(&input, 4000000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"#;

    #[test]
    fn test_first() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}._solve_first(&input, 10);
        assert_eq!(result, Some(26));
    }

    #[test]
    fn test_second() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}._solve_second(&input, 20);
        assert_eq!(result, Some(56000011));
    }
}
