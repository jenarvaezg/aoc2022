use io::Result;

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<Vec<u128>>;
    type Output = u128;

    fn parse_input<R: io::Read>(&self, r: R) -> Self::Input {
        let lines: Vec<String> = BufReader::new(r).lines().filter_map(Result::ok).collect();

        let mut result = Vec::new();

        let mut current_vec = Vec::new();
        for line in lines.iter() {
            if line.is_empty() {
                result.push(current_vec.clone());
                current_vec.clear();
            } else {
                current_vec.push(line.parse().unwrap());
            }
        }
        if !current_vec.is_empty() {
            result.push(current_vec.clone());
        }

        result
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        input.iter().map(|x| x.iter().sum::<u128>()).max().unwrap()
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        let mut calories: Vec<u128> = input.iter().map(|x| x.iter().sum::<u128>()).collect();

        calories.sort();
        println!("{:?}", calories);

        calories.iter().rev().take(3).sum::<u128>()
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

10000
"#;
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
