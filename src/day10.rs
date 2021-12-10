enum RuleViolation {
    Corrupted(char),
    Missing(Vec<char>),
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

fn determine_violation(line: &str) -> RuleViolation {
    let mut scratch = Vec::with_capacity(32);
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => scratch.push(c),
            ')' if *scratch.last().unwrap() == '(' => { scratch.pop(); }
            ']' if *scratch.last().unwrap() == '[' => { scratch.pop(); }
            '}' if *scratch.last().unwrap() == '{' => { scratch.pop(); }
            '>' if *scratch.last().unwrap() == '<' => { scratch.pop(); }
            _ => return RuleViolation::Corrupted(c)
        };
    }

    scratch.reverse();
    RuleViolation::Missing(scratch)
}

pub fn solve(input: &str) -> String {
    let violations = input.lines()
        .map(|line| determine_violation(line))
        .collect::<Vec<_>>();

    let part1 = violations.iter().filter_map(|rv| if let RuleViolation::Corrupted(c) = rv {
        Some(corrupt_score(*c))
    } else {
        None
    }).sum::<u32>();
    let mut part2_scores = violations.into_iter().filter_map(|rv| if let RuleViolation::Missing(v) = rv {
        Some(v.into_iter().fold(0, |score, next| score * 5 + incomplete_score(next)))
    } else {
        None
    }).collect::<Vec<_>>();
    part2_scores.sort();
    let part2 = part2_scores[part2_scores.len() / 2];
    format!("Sum first illegal codes: {}, middle autocorrect score: {}", part1, part2)
}