use std::fmt::{Display, Formatter};
use std::mem::swap;
use crate::bounded2d::{Array2DIndex, BoundedLinear2DArray};

#[derive(Debug)]
struct Octopuses(BoundedLinear2DArray<i32>);

fn step(data: &mut Octopuses) -> u32 {
    let data = &mut data.0;
    let mut flashes = 0;

    for idx in data.indices() {
        data[idx] += 1;
    }

    let mut check_for_next_flash = true;

    while check_for_next_flash {
        check_for_next_flash = false;

        for idx in data.indices() {
            if data[idx] > 9 {
                data[idx] = 0;
                flashes += 1;
                check_for_next_flash = true;

                for neighbour in data.diagonal_neighbours_of(idx) {
                    let current = data[neighbour];
                    if current != 0 {
                        data[neighbour] = current + 1;
                    }
                }
            }
        }
    }

    flashes
}

pub fn solve(input: &str) -> String {
    let mut data = BoundedLinear2DArray::new(
        input.chars()
            .filter_map(|c| if c >= '0' && c <= '9' { Some(c as i32 - '0' as i32) } else { None })
            .collect(),
        10,
        0,
    );


    let mut count = 0;
    let mut data = Octopuses(data);
    for i in 0..100 {
        count += step(&mut data);
    }

    let mut iterations = 101;
    while step(&mut data) != 100 {
        iterations  += 1;
    }
    format!("{} flashes, final reached in {}", count, iterations)
}