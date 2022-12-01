use crate::solver::Solver;
use std::fs::File;
mod day1;
// End imports 

fn load_day(day: u32) -> File {
    let path = format!("inputs/day{}.txt", day);
    match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path, why),
        Ok(file) => file,
    }
}

pub fn solve(day: u32) {
    let day_file = load_day(day);
    match day {
        1 => day1::Problem {}.solve(day_file),
        d => println!("Day {} has not been solved yet", d),
    }
}
