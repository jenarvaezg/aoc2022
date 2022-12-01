use io::Result;

use crate::solver::Solver;
use std::cmp::Reverse;
use std::io::{self, BufReader, Read};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<u128>;
    type Output = u128;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let mut data = String::new();
        BufReader::new(r).read_to_string(&mut data).unwrap();

        data.split("\n\n")
            .map(|lines| lines.lines().filter_map(|x| x.parse::<u128>().ok()).sum())
            .collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        *input.iter().max().unwrap()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let mut calories: Vec<u128> = input.clone();

        calories.sort_unstable_by_key(|x| Reverse(*x));

        calories.iter().take(3).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stringreader::StringReader;

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
        let input = Problem {}.parse_input(StringReader::new(input));
        let result = Problem {}.solve_first(&input);
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

10000
"#;
        let input = Problem {}.parse_input(StringReader::new(input));
        let result = Problem {}.solve_second(&input);
        assert_eq!(result, 45000);
    }
}
