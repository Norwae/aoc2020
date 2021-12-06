pub fn solve(input: &str) -> String {
    let mut populations = [0u64;9];
    for age in input.split(",").map(|str|str.trim().parse::<usize>().unwrap()) {
        populations[age] += 1
    }

    println!("Initial {:?}", &populations);
    for day in 0..256 {
        let spawners = populations[0];
        for i in 0..=7 {
            populations[i] = populations[i + 1];
        }
        populations[6] += spawners;
        populations[8] = spawners;

    }

    format!("total: {}",  populations.iter().cloned().sum::<u64>())
}