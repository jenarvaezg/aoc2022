use crate::solver::Solver;
use std::cmp::Reverse;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u128>;
    type Output = u128;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        Some(
            raw_input
                .split("\n\n")
                .map(|lines| lines.lines().filter_map(|x| x.parse::<u128>().ok()).sum())
                .collect(),
        )
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(*input.iter().max().unwrap())
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        let mut calories: Vec<u128> = input.clone();

        calories.sort_unstable_by_key(|x| Reverse(*x));

        Some(calories.iter().take(3).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
        let input = Problem {}.parse_input(String::from(input)).unwrap();
        let result = Problem {}.solve_first(&input).unwrap();
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_second() {
        let input = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;
        let input = Problem {}.parse_input(String::from(input)).unwrap();
        let result = Problem {}.solve_second(&input).unwrap();
        assert_eq!(result, 45000);
    }
}
