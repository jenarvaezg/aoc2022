use crate::solver::Solver;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u128>;
    type Output = usize;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        Some(Vec::new())
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

    const EXAMPLE: &str = r#""#;

    #[test]
    fn test_first() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, None);
    }

    #[test]
    fn test_second() {
        let input = Problem {}.parse_input(String::from(EXAMPLE)).unwrap();
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, None);
    }
}
