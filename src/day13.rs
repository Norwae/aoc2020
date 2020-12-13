use std::ops::{Add, Mul, Sub, Neg};
use std::str::FromStr;

use num::{clamp, Integer};
use num::integer::lcm;
use regex::Regex;

#[derive(Debug, Copy, Clone)]
struct ModuleValue {
    value_in_modulus: i64,
    modulus: i64,
}

macro_rules! arithmetics {
    ($tpe:ty, $name:ident, $op:tt) => {
        impl $tpe for ModuleValue {
            type Output = ModuleValue;

            fn $name(self, rhs: ModuleValue) -> ModuleValue {
                self.verify(rhs.modulus);
                ModuleValue::new(self.value_in_modulus $op rhs.value_in_modulus, self.modulus)
            }
        }
    };
}

arithmetics!(Add, add, +);
arithmetics!(Sub, sub, -);
arithmetics!(Mul, mul, *);

impl ModuleValue {
    fn new(value: i64, modulus: i64) -> Self {
        let mut value_in_modulus = value % modulus;
        if value_in_modulus < 0 {
            value_in_modulus += modulus
        }
        ModuleValue { value_in_modulus, modulus }
    }

    fn multiplicative_inverse(&self, plain: i64) -> ModuleValue {
        let value_in_modulus = plain.extended_gcd(&self.modulus).x;

        Self::new(value_in_modulus, self.modulus)
    }

    fn verify(self, m: i64) {
        if m != self.modulus {
            panic!("Cannot add in different moduli")
        }
    }
}

impl Neg for ModuleValue {
    type Output = ModuleValue;

    fn neg(self) -> Self::Output {
        ModuleValue::new(-self.value_in_modulus, self.modulus)
    }
}

pub fn solve(input: &str) {
    let parse_departures_simple = Regex::new(r"(\d+)(,|$)").unwrap();
    let parse_departures_twist = Regex::new(r"(\d+|x)(,|$)").unwrap();
    let mut lines = input.lines();
    let line1 = lines.next().unwrap();
    println!("line: {}", line1);
    let arrival = i64::from_str(line1).unwrap();

    let best = parse_departures_simple.captures_iter(input).map(|d| {
        let departure = i64::from_str(d.get(1).unwrap().as_str()).unwrap();
        (departure - (arrival % departure), departure)
    }).min().unwrap();
    println!("Best departure: {:?}", best.0 * best.1);

    let target = parse_departures_twist
        .captures_iter(input)
        .map(|d| {
            i64::from_str(d.get(1).unwrap().as_str()).ok()
        })
        .enumerate()
        .filter_map(|t| t.1.map(|inner| ModuleValue {
            modulus: inner,
            value_in_modulus: t.0 as i64,
        }))
        .collect::<Vec<_>>();
    println!("Deltas: {:?}", target);
    find_alignment(&target.as_slice()[1..], 0, target[0].modulus);
}

fn find_alignment(targets: &[ModuleValue], first: i64, stride: i64) {
    if targets.is_empty() {
        println!("Found match at {}", first);
        return
    }
    /*
    mod target.modulus:
    start + target.value_in_module + a * stride = 0
    a * stride = -(start + target.value_in_module) (:= k)
    a = k * invert(stride)
     */

    let target = targets[0];
    let start = ModuleValue::new(first, target.modulus);
    let k = -(start + target);
    let invert = k.multiplicative_inverse(stride);
    let a = k * invert;

    find_alignment(&targets[1..], first + a.value_in_modulus * stride, stride * target.modulus);
}


pub const EXAMPLE_INPUT: &str = "939
7,13,x,x,59,x,31,19";

pub const INPUT: &str = "1000509
17,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,739,x,29,x,x,x,x,x,x,x,x,x,x,13,x,x,x,x,x,x,x,x,x,23,x,x,x,x,x,x,x,971,x,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,19";

