use std::error::Error;
use std::fmt::{self, Formatter};
use std::ops::{Index, Mul, Not};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct NBitNr {
    bits: usize,
    value: u16,
}

impl fmt::Display for NBitNr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("{value:0bits$b}", value = self.value, bits = self.bits))
    }
}

impl Not for NBitNr {
    type Output = NBitNr;

    fn not(self) -> Self::Output {
        let bits = self.bits;
        let mask = 0xffffu16 >> (16 - bits);
        let value = (!self.value) & mask;
        Self { bits, value }
    }
}

impl<I: Iterator<Item=bool>> From<I> for NBitNr {
    fn from(v: I) -> Self {
        let mut bits = 0;
        let mut value = 0;
        for flag in v {
            bits += 1;
            value = (value << 1) | (flag as u16);
        }

        Self { bits, value }
    }
}

impl Mul for NBitNr {
    type Output = u32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.value as u32 * rhs.value as u32
    }
}

static TRUE: bool = true;
static FALSE: bool = false;

impl Index<usize> for NBitNr {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.bits);
        let mask = 1u16 << (self.bits - index - 1);
        if (self.value & mask) != 0 {
            &TRUE
        } else {
            &FALSE
        }
    }
}

impl NBitNr {
    fn from(input: &str) -> Result<NBitNr, Box<dyn Error>> {
        let bits = input.len();
        let value = u16::from_str_radix(input, 2)?;
        Ok(NBitNr { value, bits })
    }
}

fn count_ones(input: &Vec<NBitNr>) -> Vec<usize> {
    let nr_length = input[0].bits;
    let mut v = vec![0usize; nr_length];

    for nr in input {
        for i in 0..nr_length {
            v[i] += nr[i] as usize
        }
    }

    v
}

fn progressive_search<F: Fn(usize, usize) -> bool>(haystack: &Vec<NBitNr>, condition: F) -> NBitNr {
    let nr_length = haystack[0].bits;
    let mut haystack = haystack.clone();

    for check_idx in 0..=nr_length {
        let ones = haystack.iter().fold(0usize, |sum, next| sum + (next[check_idx] as usize));
        let next_check_bit = condition(haystack.len() - ones, ones);
        dbg!(check_idx, haystack.len() - ones, ones, condition(haystack.len() - ones, ones));

        haystack = haystack.into_iter().filter(|element| element[check_idx] == next_check_bit).collect();

        if haystack.len() == 1 {
            break;
        }

    }

    assert_eq!(haystack.len(), 1);
    let value = haystack[0];
    println!("extracted {} from haystack", &value);
    value

}

pub fn solve(input: &str) -> String {
    let numbers = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|v| NBitNr::from(v).unwrap())
        .collect::<Vec<_>>();
    let one_counts = count_ones(&numbers);
    let gamma: NBitNr = one_counts.iter().map(|v| *v > numbers.len() / 2).into();
    let epsilon = !gamma;

    let ogr = progressive_search(&numbers, |zeros, ones| zeros <= ones);
    let csr = progressive_search(&numbers, |zeros, ones| zeros > ones);

    format!("gamma * epsilon: {} - ogr * csr: {}", gamma * epsilon, ogr * csr)
}