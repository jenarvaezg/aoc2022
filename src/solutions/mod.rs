use crate::solver::Solver;
use std::error::Error;
use std::fs;
mod day1;
mod day2;
mod day3;
mod day4;
// End imports  

fn load_day(day: u32) -> Result<String, Box<dyn Error>> {
    Ok(fs::read_to_string(format!("inputs/day{}.txt", day))?)
}

pub fn solve(day: u32) {
    let raw_input = load_day(day).expect("Problem parsing day input");
    match day {
        1 => day1::Problem {}.solve(raw_input),
        2 => day2::Problem {}.solve(raw_input),
        3 => day3::Problem {}.solve(raw_input),
        4 => day4::Problem {}.solve(raw_input),
        d => println!("Day {} has not been solved yet", d),
    }
}
