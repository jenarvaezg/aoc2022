use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
};

use crate::solver::Solver;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
pub struct Movement {
    direction: Direction,
    steps: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    x: isize,
    y: isize,
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)?;
        Ok(())
    }
}

impl Coord {
    fn is_adjacent_to(&self, other: &Self) -> bool {
        self.x.abs_diff(other.x) < 2 && self.y.abs_diff(other.y) < 2
    }

    fn move_once(self, direction: &Direction) -> Self {
        match direction {
            Direction::Down => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Up => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }

    fn approach(self, other: &Self) -> Self {
        // Assume not adjacent
        let new_x = self.x + (other.x - self.x).signum();
        let new_y = self.y + (other.y - self.y).signum();
        return Self { x: new_x, y: new_y };
    }
}

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Movement>;
    type Output = usize;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        Some(
            raw_input
                .lines()
                .map(|l| {
                    let (direction, steps) = l.split_once(" ").unwrap();
                    let direction = match direction {
                        "R" => Direction::Right,
                        "L" => Direction::Left,
                        "D" => Direction::Down,
                        "U" => Direction::Up,
                        _ => unreachable!(),
                    };
                    Movement {
                        direction,
                        steps: steps.parse().unwrap(),
                    }
                })
                .collect(),
        )
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut visited = HashSet::<Coord>::new();
        let mut head_position = Coord { x: 0, y: 0 };
        let mut tail_position = Coord { x: 0, y: 0 };

        for movement in input.into_iter() {
            for _ in 0..movement.steps {
                (head_position, tail_position) =
                    move_rope(head_position, tail_position, &movement.direction);
                visited.insert(tail_position);
            }
        }
        Some(visited.len())
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut visited = HashSet::<Coord>::new();
        let mut rope = vec![Coord { x: 0, y: 0 }; 10];

        for movement in input.into_iter() {
            for _ in 0..movement.steps {
                move_long_rope(&mut rope, &movement.direction);
                visited.insert(*rope.last().unwrap());
            }
        }
        Some(visited.len())
    }
}

fn move_rope(head_position: Coord, tail_position: Coord, direction: &Direction) -> (Coord, Coord) {
    let new_head_position = head_position.move_once(&direction);

    let new_tail_position = match tail_position.is_adjacent_to(&new_head_position) {
        true => tail_position,
        false => tail_position.approach(&new_head_position),
    };

    (new_head_position, new_tail_position)
}

fn move_long_rope(rope: &mut Vec<Coord>, direction: &Direction) {
    let new_head_position = rope.first().unwrap().move_once(&direction);
    rope[0] = new_head_position;

    for i in 1..rope.len() {
        let (head, tail) = (rope[i - 1], rope[i]);
        let new_tail = match tail.is_adjacent_to(&head) {
            true => tail,
            false => tail.approach(&head),
        };

        rope[i] = new_tail;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2"#;

    #[test]
    fn test_first() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_second() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_second_complex() {
        let example = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"#;
        let input = Problem {}.parse_input(String::from(example)).unwrap();
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, Some(36));
    }
}
