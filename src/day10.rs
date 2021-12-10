use std::borrow::BorrowMut;
use std::cell::{RefCell, UnsafeCell};
use std::ops::Range;

struct ScratchPool {
    storage: Vec<char>,
    watermark: usize
}

struct PoolToken(Range<usize>);

impl ScratchPool {
    fn new() -> Self {
        let storage = Vec::with_capacity(4096);
        let watermark = 0;
        Self { storage, watermark }
    }

    fn push(&mut self, c: char) {
        self.storage.push(c)
    }

    fn pop(&mut self) {
        self.storage.pop();
    }

    fn peek(&self) -> char {
        self.storage[self.storage.len() - 1]
    }

    fn slice(&mut self) -> PoolToken {
        let token = PoolToken(self.watermark..self.storage.len());
        self.watermark = self.storage.len();
        token
    }

    fn reset(&mut self) {
        self.storage.truncate(self.watermark);
    }

    fn view(&self, token: PoolToken)->&[char] {
        &self.storage[token.0]
    }
}

enum RuleViolation {
    Corrupted(char),
    Missing(PoolToken),
}

fn corrupt_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Invalid character '{}'", c)
    }
}

fn incomplete_score(c: char) -> u64 {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("Invalid character '{}'", c)
    }
}

fn determine_violation(line: &str, scratch: &mut ScratchPool) -> RuleViolation {
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => scratch.push(c),
            ')' if scratch.peek() == '(' => scratch.pop(),
            ']' if scratch.peek() == '[' =>  scratch.pop(),
            '}' if scratch.peek() == '{' =>  scratch.pop(),
            '>' if scratch.peek() == '<' =>  scratch.pop(),
            _ => {
                scratch.reset();
                return RuleViolation::Corrupted(c)
            }
        };
    }

    RuleViolation::Missing(scratch.slice())
}

pub fn solve(input: &str) -> String {
    let mut pool = ScratchPool::new();
    let mut violations = Vec::new();
    for line in input.lines(){
        violations.push(determine_violation(line, &mut pool))
    }

    let part1 = violations.iter().filter_map(|rv| if let RuleViolation::Corrupted(c) = rv {
        Some(corrupt_score(*c))
    } else {
        None
    }).sum::<u32>();
    let mut part2_scores = violations.into_iter().filter_map(|rv| if let RuleViolation::Missing(token) = rv {
        Some(pool.view(token).into_iter().rev().fold(0, |score, next| score * 5 + incomplete_score(*next)))
    } else {
        None
    }).collect::<Vec<_>>();
    part2_scores.sort();
    let part2 = part2_scores[part2_scores.len() / 2];
    format!("Sum first illegal codes: {}, middle autocorrect score: {}", part1, part2)
}