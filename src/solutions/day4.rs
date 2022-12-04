use crate::solver::Solver;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Problem;

lazy_static! {
    static ref ASSIGNMENTS_RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
}

impl Solver for Problem {
    type Input = Vec<((u128, u128), (u128, u128))>;
    type Output = u128;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        Some(
            raw_input
                .lines()
                .map(|line| {
                    let captures = ASSIGNMENTS_RE.captures(line).unwrap();

                    (
                        (captures[1].parse().unwrap(), captures[2].parse().unwrap()),
                        (captures[3].parse().unwrap(), captures[4].parse().unwrap()),
                    )
                })
                .collect(),
        )
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .into_iter()
                .filter(|(one, other)| {
                    (one.0 <= other.0 && one.1 >= other.1) || (other.0 <= one.0 && other.1 >= one.1)
                })
                .count() as u128,
        )
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .into_iter()
                .filter(|(one, other)| one.0 <= other.1 && other.0 <= one.1)
                .count() as u128,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn test_first() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        println!("{:?}", input);
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_second() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, Some(4));
    }
}
