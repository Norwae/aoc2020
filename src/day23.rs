const STEPS: usize = 10_000_000;
const SIZE: usize = 1_000_000;

pub fn solve() {
    let mut cups = [4usize,8,7,9,1,2,3,6,5].to_vec();
    cups.reserve(SIZE - cups.len());

    for i in cups.len() + 1..=SIZE {
        cups.push(i as usize);
    }

    let (f, ff) = run_game(&cups, STEPS);
    println!("{} * {} = {}", f, ff, f * ff);
}

fn run_game(cups: &[usize], steps: usize) -> (usize, usize) {
    let mut next = build_next(cups);
    let mut current = cups[0];

    for _ in 0..steps {
        let pickup = [next[current], next[next[current]], next[next[next[current]]]];
        let next_available = find_next_index(cups.len(), current, &pickup[..]);

        next.swap(next_available, current);
        next.swap(current, pickup[pickup.len() - 1]);

        current = next[current];
    }

    (next[1], next[next[1]])
}

fn find_next_index(len: usize, current: usize, pickup: &[usize]) -> usize {
    let mut i = 1usize;
    loop {
        let seek = if current > i {
            current - i
        } else {
            len + current - i
        };

        if !pickup.contains(&seek) {
            return seek;
        }

        i += 1;
    }
}

fn build_next(cups: &[usize]) -> Vec<usize> {
    let mut next = vec![0; cups.len() + 1]; // wastes index 0, but who cares?
    for i in 0..cups.len() - 1 {
        next[cups[i]] = cups[i + 1];
    }
    next[cups[cups.len() - 1]] = cups[0];
    next
}