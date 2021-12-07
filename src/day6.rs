pub fn solve(input: &str) -> String {
    let mut populations = [0u64;9];
    for age in input.split(",").map(|str|str.trim().parse::<usize>().unwrap()) {
        populations[age] += 1
    }

    for day in 0..256 {
        populations[(day + 7) % 9] += populations[day % 9];
    }

    format!("total: {}",  populations.iter().cloned().sum::<u64>())
}