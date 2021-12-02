use std::time;
use std::io;

pub fn run_with_timing<F: FnOnce(&str) -> String>(body: F) -> Result<(), io::Error> {
    let buffer = read_problem_input()?;

    let start = time::Instant::now();
    let result = body(&buffer);
    let end = time::Instant::now();

    println!("Computation took: {:?} and yielded \"{}\"" , (end - start), result);
    Ok(())
}

fn read_problem_input() -> Result<String, io::Error> {
    println!("Input problem data, terminate with empty line or eof");
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut lines = 0;

    loop {
        let read = stdin.read_line(&mut buffer)?;
        if read == 0 || buffer.ends_with("\r\n\r\n") || buffer.ends_with("\n\n") {
            break
        }
        lines += 1
    }

    println!("Read {} bytes of input ({} lines of text)", buffer.as_bytes().len(), lines);
    Ok(buffer)
}