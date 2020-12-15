use std::collections::HashMap;
use std::str::FromStr;

pub fn solve(input: &str) {
    let mut numbers = input
        .split(',')
        .map(|s| u64::from_str(s).unwrap())
        .collect::<Vec<_>>();
    let mut ledger = numbers
        .iter()
        .enumerate()
        .map(|(x, y)| (*y, x + 1))
        .take(numbers.len() - 1)
        .collect::<HashMap<_, _>>();

    while numbers.len() < 30000000 {
        let previous_number = numbers.last().unwrap();
        let current_index = numbers.len();
        let number_to_say = match ledger.get(previous_number) {
            None => 0,
            Some(already_seen) => current_index - already_seen
        } as u64;

        ledger.insert(*previous_number, current_index);
        numbers.push(number_to_say);
    }
    println!("{:?} with max {}", numbers.last());
}

pub const EXAMPLE_INPUT: &str = "0,3,6";
pub const INPUT: &str = "9,6,0,10,18,2,1";