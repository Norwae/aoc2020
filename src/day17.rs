use std::collections:: BTreeSet;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
struct Cube {
    active: BTreeSet<Coordinate>
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Coordinate { w: i64, z: i64, y: i64, x: i64 }

impl Coordinate {
    fn shift(self, delta: i64) -> Self {
        Coordinate { w: self.w + delta, x: self.x + delta, y: self.y + delta, z: self.z + delta }
    }

    fn neighbourhood(self) -> impl Iterator<Item=Coordinate> {
        fn generate_deltas()-> [(i64, i64, i64); 26] {
            let mut idx = 0;
            let mut result = [(0,0,0); 26];
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        if dx != 0 || dy != 0 || dz != 0 {
                            result[idx] = (dx, dy, dz);
                            idx += 1
                        }
                    }
                }
            }

            result
        }

        lazy_static! {
            static ref DELTAS: [(i64, i64,i64); 26] = generate_deltas();
        }
        struct Iter(Coordinate, usize);

        impl Iterator for Iter {
            type Item = Coordinate;

            fn next(&mut self) -> Option<Self::Item> {
                if self.1 >= DELTAS.len() {
                    None
                } else {
                    let Coordinate { w, x, y, z } = self.0;
                    let (dx, dy, dz) = DELTAS[self.1];
                    self.1 += 1;
                    Some(Coordinate { w, x: x + dx, y: y + dy, z: z + dz })
                }
            }
        }

        Iter(self, 0)
    }
}

impl Cube {
    fn new() -> Self {
        Self { active: BTreeSet::new() }
    }

    fn potentially_active_squares(&self) -> impl Iterator<Item = Coordinate> {
        let (start, end) = self.bounds();
        let start = start.shift(-1);
        let end = end.shift(1);
        struct Iter {
            current: Coordinate,
            start: Coordinate,
            end: Coordinate
        }

        impl Iterator for Iter {
            type Item = Coordinate;

            fn next(&mut self) -> Option<Self::Item> {
                if self.current > self.end {
                    None
                } else {
                    let next = self.current;
                    self.current.x += 1;
                    if self.current.x > self.end.x {
                        self.current.x = self.start.x;
                        self.current.y += 1;
                        if self.current.y > self.end.y {
                            self.current.y = self.start.y;
                            self.current.z += 1;
                            if self.current.z > self.end.z {
                                self.current.z = self.start.z;
                                self.current.w += 1;
                            }
                        }
                    }

                    Some(next)
                }
            }
        }

        Iter { start, end, current: start }
    }

    fn step(&self) -> Self {
        let mut active = BTreeSet::new();
        for coord in self.potentially_active_squares() {
            let count = coord.neighbourhood().filter(|c| self.active.contains(&c)).count();

            if count >= 2 {
                if self.active.contains(&coord) && count <= 3 {
                    active.insert(coord);
                }
                if !self.active.contains(&coord) && count == 3 {
                    active.insert(coord);
                }
            }
        }
        Cube { active }
    }

    fn bounds(&self) -> (Coordinate, Coordinate) {
        self.active.iter().fold(
            (
                Coordinate{ w: i64::MAX, z: i64::MAX, y: i64::MAX, x: i64::MAX},
                Coordinate{ w: i64::MIN, z: i64::MIN, y: i64::MIN, x: i64::MIN},
            ), |(min, max), next| {
                (Coordinate {
                    w: next.w.min(min.w),
                    x: next.x.min(min.x),
                    y: next.y.min(min.y),
                    z: next.z.min(min.z),
                }, Coordinate {
                    w: next.w.max(max.w),
                    x: next.x.max(max.x),
                    y: next.y.max(max.y),
                    z: next.z.max(max.z),
                })
            },
        )
    }

    fn load_initial_layer(&mut self, input: &str) {
        input.split_whitespace().enumerate().for_each(|(row_idx, row)| {
            row.bytes().enumerate().for_each(|(col_idx, col)| {
                if col == b'#' {
                    self.active.insert(Coordinate { w: 0, x: col_idx as i64, y: row_idx as i64, z: 0});
                }
            })
        })
    }
}

pub fn solve(input: &str) {
    let mut cube = Cube::new();
    cube.load_initial_layer(input);

    for _ in 0..6 {
        cube = cube.step()
    }
    println!("After 6 steps active: {}", cube.active.len())
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