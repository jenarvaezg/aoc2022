import sys
import os
from pathlib import Path
import requests

day_template = """
use io::Result;

use crate::solver::Solver;
use std::io::{self, BufRead, BufReader};

pub struct Problem;

impl Solver for Problem {
    type Input = Vec<String>;
    type Output = usize;

    fn parse_input<R: io::Read + io::Seek>(&self, r: R) -> Self::Input {
        BufReader::new(r).lines().filter_map(Result::ok).collect()
    }

    fn solve_first(&self, input: &Self::Input) -> Self::Output {
        1
    }

    fn solve_second(&self, input: &Self::Input) -> Self::Output {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let result = Problem {}.solve_first(&Vec::new());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_second() {
        let result = Problem {}.solve_second(&Vec::new());
        assert_eq!(result, 2);
    }
}
"""


def get_input(day: int, year: int = 2022) -> str:
    cookie = os.environ["AOC_SESSION"]

    return requests.get(
        f"https://adventofcode.com/{year}/day/{day}/input", cookies={"session": cookie}
    ).text


def main():
    day = sys.argv[1]
    # Create day file
    day_path = Path(f"src/solutions/day{day}.rs")
    if day_path.exists():
        print(f"Day {day} already set")
        exit(1)
    day_path.write_text(day_template)

    # Attach day
    mod_path = Path(f"src/solutions/mod.rs")
    text = mod_path.read_text()
    text = text.replace("// End imports", f"mod day{day};\n// End imports ")
    text = text.replace(
        "d => ", f"{day} => day{day}::Problem {{}}.solve(day_file),\n        d => "
    )
    mod_path.write_text(text)

    # Get input
    day_input = get_input(day)
    input_path = Path(f"inputs/day{day}.txt")
    input_path.write_text(day_input)


if __name__ == "__main__":
    main()
