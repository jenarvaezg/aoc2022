use std::{fmt::Display, time::Instant};

pub trait Solver {
    type Input;
    type Output: Display;

    fn parse_input(&self, input_str: String) -> Option<Self::Input>;
    fn solve_first(&self, input: &Self::Input) -> Option<Self::Output>;
    fn solve_second(&self, input: &Self::Input) -> Option<Self::Output>;

    fn timed_first(&self, input: &Self::Input) -> Option<Self::Output> {
        let before = Instant::now();
        let solution = self.solve_first(input)?;

        println!("Part 1: {:?}", before.elapsed());
        Some(solution)
    }

    fn timed_second(&self, input: &Self::Input) -> Option<Self::Output> {
        let before = Instant::now();
        let solution = self.solve_second(input)?;

        println!("Part 2: {:?}", before.elapsed());
        Some(solution)
    }

    fn solve(&self, raw_input: String) {
        let input = self.parse_input(raw_input).expect("Unable to parse input");
        if let Some(s1) = self.timed_first(&input) {
            println!("Solution 1: {}", s1);
        }
        println!("");
        if let Some(s2) = self.timed_second(&input) {
            println!("Solution 2: {}", s2);
        }
    }
}
