use std::collections::HashSet;

use crate::solver::Solver;

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<char>;
    type Output = u128;

    fn parse_input(&self, raw_input: String) -> Option<Self::Input> {
        Some(raw_input.chars().collect())
    }

    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .as_slice()
                .windows(4)
                .enumerate()
                .find(|(_i, x)| x.iter().collect::<HashSet<_>>().len() == x.len())
                .unwrap()
                .0 as u128
                + 4,
        )
    }

    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output> {
        Some(
            input
                .as_slice()
                .windows(14)
                .enumerate()
                .find(|(_i, x)| x.iter().collect::<HashSet<_>>().len() == x.len())
                .unwrap()
                .0 as u128
                + 14,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"bvwbjplbgvbhsrlpgdmjqwftvncz"#;
    const EXAMPLE2: &str = r#"nppdvjthqldpwncqszvftbrmjlhg"#;
    const EXAMPLE3: &str = r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"#;
    const EXAMPLE4: &str = r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"#;

    #[test]
    fn test_first_1() {
        let input = Problem {}.parse_input(String::from(EXAMPLE1)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_first_2() {
        let input = Problem {}.parse_input(String::from(EXAMPLE2)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_first_3() {
        let input = Problem {}.parse_input(String::from(EXAMPLE3)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_first_4() {
        let input = Problem {}.parse_input(String::from(EXAMPLE4)).unwrap();
        let result = Problem {}.solve_first(&input);
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_second() {
        let input = Problem {}.parse_input(String::from(EXAMPLE1)).unwrap();
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, Some(23));
    }
}
