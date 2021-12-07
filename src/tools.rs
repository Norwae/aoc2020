use std::time;
use std::io;

pub fn run_with_timing<F: Fn(&str) -> String>(body: F) -> Result<(), io::Error> {
    println!("Input problem data, terminate single run with !, exit with !!");

    loop {
        let buffer = read_problem_input()?;
        if buffer.ends_with("!") {
            return Ok(())
        }

        let start = time::Instant::now();
        let result = body(&buffer);
        let end = time::Instant::now();

        println!("Computation took: {:?} and yielded \"{}\"", (end - start), result);
    }
}

fn read_problem_input() -> Result<String, io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut lines = 0;

    loop {
        let read = stdin.read_line(&mut buffer)?;
        if read == 0 || buffer.ends_with("!\n") {
            break
        }
        lines += 1
    }

    println!("Read {} bytes of input ({} lines of text)", buffer.as_bytes().len(), lines);
    buffer.truncate(buffer.len() - 2);
    Ok(buffer)
}