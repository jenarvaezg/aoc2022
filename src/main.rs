mod grid;
mod solutions;
mod solver;
use std::env;

fn main() {
    let day: u32 = env::args()
        .nth(1)
        .expect("Day is required")
        .parse()
        .unwrap();
    println!("Running day {}", day);
    solutions::solve(day);
}
