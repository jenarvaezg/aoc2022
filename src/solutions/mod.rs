use crate::solver::Solver;
use std::error::Error;
use std::fs;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
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
        5 => day5::Problem {}.solve(raw_input),
        6 => day6::Problem {}.solve(raw_input),
        7 => day7::Problem {}.solve(raw_input),
        8 => day8::Problem {}.solve(raw_input),
        9 => day9::Problem {}.solve(raw_input),
        10 => day10::Problem {}.solve(raw_input),
        11 => day11::Problem {}.solve(raw_input),
        12 => day12::Problem {}.solve(raw_input),
        13 => day13::Problem {}.solve(raw_input),
        14 => day14::Problem {}.solve(raw_input),
        15 => day15::Problem {}.solve(raw_input),
        16 => day16::Problem {}.solve(raw_input),
        d => println!("Day {} has not been solved yet", d),
    }
}
