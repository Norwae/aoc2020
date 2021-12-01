use crate::tools::run_with_timing;

mod tools;

mod day1;

fn main() -> Result<(), std::io::Error>{
    run_with_timing(|s| day1::solve(s))
}
