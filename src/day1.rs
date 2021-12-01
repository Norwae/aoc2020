
pub fn solve(input: &str) -> String {
    let numbers: Vec<i32> = input.lines()
        .filter(|x|x.len() > 0)
        .map(|slice|slice.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let increases = numbers
        .windows(2)
        .filter(|s| s[0] < s[1])
        .count();


    let increased_averages = numbers
        .windows(3)
        .map(|s| s[0] + s[1] + s[2])
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|s|s[0] < s[1])
        .count();
    format!("The depth increased {} times naively and {} in 3-step windows", increases, increased_averages)
}