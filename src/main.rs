use crate::tools::run_with_timing;

use std::error::Error;

mod tools;

mod day1;
mod day2;

const SOLVERS: [for<'r> fn(&'r str) -> String; 2] = [
    day1::solve,
    day2::solve
];

fn main() -> Result<(), Box<dyn Error>>{
    println!("Input day to solve: ");
    let mut buffer = String::new();

    std::io::stdin().read_line(&mut buffer)?;
    let problem_nr = buffer.trim().parse::<usize>()?;
    run_with_timing(SOLVERS[problem_nr - 1])?;
    Ok(())
}
