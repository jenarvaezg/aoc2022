use crate::solver::Solver;
use std::cmp::Reverse;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u128>;
    type Output = u128;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        Vec::new()
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        None
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stringreader::StringReader;

    #[test]
    fn test_first() {
        let input = r#""#;
        let input = Problem {}.parse_input(String::from(input)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_second() {
        let input = r#""#;
        let input = Problem {}.parse_input(String::from(input)).unwrap();
        let result = Problem {}.solve_second(&input).unwrap();
        assert_eq!(result, None);
    }
}
