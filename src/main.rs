use crate::tools::run_with_timing;

use std::error::Error;

mod tools;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn fail(_: &str) -> String {
    unimplemented!()
}

const SOLVERS: [for<'r> fn(&'r str) -> String; 25] = [
    day1::solve,
    day2::solve,
    day3::solve,
    day4::solve,
    day5::solve,
    day6::solve,
    day7::solve,
    day8::solve,
    fail,
    fail,
    fail,
    fail,
    fail,
    fail,
    fail,
    fail,
    fail,
    fail,
    fail,
    fail,
    fail,
    fail,
    fail,
    fail,
    fail
];

fn main() -> Result<(), Box<dyn Error>> {
    println!("Input day to solve: ");
    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer)?;
    let problem_nr = buffer.trim().parse::<usize>()?;
    run_with_timing(SOLVERS[problem_nr - 1])?;
    Ok(())
}
