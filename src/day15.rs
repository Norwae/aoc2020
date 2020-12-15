use std::str::FromStr;

pub fn solve(input: &str) {
    let mut numbers = input
        .split(',')
        .map(|s| u64::from_str(s).unwrap())
        .collect::<Vec<_>>();
    let mut ledger_vec = Vec::new();
    ledger_vec.resize(65536, -1i32);
    for i in 0..numbers.len() -1 {
        ledger_vec[numbers[i] as usize] = (i + 1) as i32;
    }

    while numbers.len() < 30000000 {
        let previous_number = *numbers.last().unwrap() as usize;
        if ledger_vec.len() <= previous_number {
            ledger_vec.resize(2 * ledger_vec.len(), -1);
        }
        let current_index = numbers.len();
        let number_to_say = if ledger_vec[previous_number] == -1 {
            0
        } else {
            (current_index - ledger_vec[previous_number] as usize)
        } as u64;

        ledger_vec[previous_number] = current_index as i32;
        numbers.push(number_to_say);
    }
    println!("{:?} with max", numbers.last());
}

pub const EXAMPLE_INPUT: &str = "0,3,6";
pub const INPUT: &str = "9,6,0,10,18,2,1";