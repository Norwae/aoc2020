fn detect_decreases(input: &Vec<i32>) -> usize {
    input
        .windows(2)
        .filter(|s| s[0] < s[1])
        .count()
}

pub fn solve(input: &str) -> String {
    let numbers: Vec<i32> = input.lines()
        .filter(|x|x.len() > 0)
        .map(|slice|slice.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let increases = detect_decreases(&numbers);

    let increased_averages = detect_decreases(&numbers.windows(3).map(|s| s[0] + s[1] + s[2]).collect());
    format!("The depth increased {} times naively and {} in 3-step windows", increases, increased_averages)
}