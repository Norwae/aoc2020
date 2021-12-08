use std::cmp::min;

fn cost_quadratic(numbers: &[i32], target: i32) -> i32 {
    numbers.iter().map(|n| {
        let steps = (n - target).abs();
        let fuel = (steps * (steps + 1)) / 2;
        fuel
    }).sum()
}

pub fn solve(input: &str) -> String {
    let mut numbers = input.split(",").map(|s|{
        s.trim().parse::<i32>().unwrap()
    }).collect::<Vec<_>>();
    numbers.sort();
    let median = numbers[numbers.len() / 2];

    let fuel_linear: i32 = numbers.iter().map(|n| (n - median).abs()).sum();
    let average = numbers.iter().sum::<i32>() as f64 / numbers.len() as f64;
    let lower = average.floor() as i32;
    let higher = average.ceil() as i32;

    let n = cost_quadratic(&numbers, lower);
    let m = cost_quadratic(&numbers, higher);
    let fuel_quadratic  = min(n, m);

    format!("Fuel needed: {} linear, {} quadratic", fuel_linear, fuel_quadratic)
}