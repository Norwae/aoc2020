use std::collections::{BTreeMap, BTreeSet};
use std::collections::hash_map::Iter;
use std::fmt::{Display, Formatter, Write};
use std::ops::{Add, Bound, Index, IndexMut};

#[derive(Debug, Clone)]
struct Cube {
    active: BTreeSet<Coordinate>
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Coordinate { z: i64, y: i64, x: i64 }

impl Coordinate {
    fn new(x: i64, y: i64, z: i64) -> Self { Self { x, y, z } }

    fn shift(self, delta: i64) -> Self {
        Coordinate { x: self.x + delta, y: self.y + delta, z: self.z + delta}
    }

    fn neighbourhood(self) -> impl Iterator<Item=Coordinate> {
        let Coordinate { x, y, z } = self;
        NeighbourhoodIterator([
                                  Coordinate::new(x - 1, y - 1, z - 1),
                                  Coordinate::new(x - 1, y - 1, z),
                                  Coordinate::new(x - 1, y - 1, z + 1),
                                  Coordinate::new(x - 1, y, z - 1),
                                  Coordinate::new(x - 1, y, z),
                                  Coordinate::new(x - 1, y, z + 1),
                                  Coordinate::new(x - 1, y + 1, z - 1),
                                  Coordinate::new(x - 1, y + 1, z),
                                  Coordinate::new(x - 1, y + 1, z + 1),
                                  Coordinate::new(x, y - 1, z - 1),
                                  Coordinate::new(x, y - 1, z),
                                  Coordinate::new(x, y - 1, z + 1),
                                  Coordinate::new(x, y, z - 1),
                                  Coordinate::new(x, y, z + 1),
                                  Coordinate::new(x, y + 1, z - 1),
                                  Coordinate::new(x, y + 1, z),
                                  Coordinate::new(x, y + 1, z + 1),
                                  Coordinate::new(x + 1, y - 1, z - 1),
                                  Coordinate::new(x + 1, y - 1, z),
                                  Coordinate::new(x + 1, y - 1, z + 1),
                                  Coordinate::new(x + 1, y, z - 1),
                                  Coordinate::new(x + 1, y, z),
                                  Coordinate::new(x + 1, y, z + 1),
                                  Coordinate::new(x + 1, y + 1, z - 1),
                                  Coordinate::new(x + 1, y + 1, z),
                                  Coordinate::new(x + 1, y + 1, z + 1)], 0)
    }
}

struct NeighbourhoodIterator([Coordinate; 26], usize);

impl Iterator for NeighbourhoodIterator {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 < 26 {
            self.1 += 1;
            Some(self.0[self.1 - 1])
        } else {
            None
        }
    }
}

impl Display for Cube {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (top, bottom) = self.bounds();

        println!("Bounds are {:?}:{:?}", top, bottom);
        for layer in top.z..=bottom.z {
            f.write_fmt(format_args!("Layer {}:", layer))?;
            f.write_char('\n')?;
            for row in top.y..=bottom.y {
                for col in top.x..=bottom.x {
                    f.write_char(
                        if self.active.contains(&Coordinate::new(col, row, layer)) { '#' } else { '.' }
                    )?
                }

                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}

impl Cube {
    fn new() -> Self {
        Self { active: BTreeSet::new() }
    }

    fn step(&self) -> Self {
        let mut active = BTreeSet::new();
        let (min, max) = self.bounds();
        let min = min.shift(-1);
        let max = max.shift(1);

        for layer in min.z..=max.z {
            for row in min.y..=max.y {
                for col in min.x..=max.x {
                    let coord = Coordinate::new(col, row, layer);
                    let count = coord.neighbourhood().filter(|c|self.active.contains(&c)).count();

                    if count >= 2 {
                        if self.active.contains(&coord) && count <= 3 {
                            active.insert(coord);
                        }
                        if !self.active.contains(&coord) && count == 3 {
                            active.insert(coord);
                        }
                    }
                }
            }
        }

        Cube { active }
    }

    fn bounds(&self) -> (Coordinate, Coordinate) {
        self.active.iter().fold(
            (
                Coordinate::new(i64::MAX, i64::MAX, i64::MAX),
                Coordinate::new(i64::MIN, i64::MIN, i64::MIN),
            ), |(min, max), next| {
                (Coordinate::new(
                    next.x.min(min.x),
                    next.y.min(min.y),
                    next.z.min(min.z)
                ), Coordinate::new(
                    next.x.max(max.x),
                    next.y.max(max.y),
                    next.z.max(max.z)
                ))
            }
        )
    }

    fn load_initial_layer(&mut self, input: &str) {
        input.split_whitespace().enumerate().for_each(|(row_idx, row)| {
            println!("Row: {}", row);
            row.bytes().enumerate().for_each(|(col_idx, col)| {
                println!("Col: {}", col);
                if col == b'#' {
                    self.active.insert(Coordinate::new(col_idx as i64, row_idx as i64, 0));
                    println!("Inserted")
                }
            })
        })
    }
}

pub fn solve(input: &str) {
    let mut step_0 = Cube::new();
    step_0.load_initial_layer(input);

    println!("Round 0: {}", &step_0);
    let step1 = step_0.step();
    println!("Round 1: {}", &step1);
    let step2 = step1.step();
    let step3 = step2.step();
    let step4 = step3.step();
    let step5 = step4.step();
    let step6 = step5.step();
    println!("After 6 steps active: {}", step6.active.len())
}

pub const INPUT: &str = "...#..#.
.....##.
##..##.#
#.#.##..
#..#.###
...##.#.
#..##..#
.#.#..#.
";

pub const EXAMPLE_INPUT: &str = ".#.
..#
###
";