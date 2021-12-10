use std::fmt::Debug;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Array2DIndex(i32, i32);

#[derive(Debug, Clone, Eq, PartialEq)]
enum Drainage {
    Unknown,
    Ridge,
    DrainsTo(Array2DIndex),
    Pending,
}

#[derive(Debug)]
struct BoundedLinear2DArray<T> {
    storage: Vec<T>,
    boundary: T,
    columns: usize,
    rows: usize,
}

impl<T> BoundedLinear2DArray<T> {
    fn new(storage: Vec<T>, columns: usize, boundary: T) -> Self {
        let rows = storage.len() / columns;
        Self { storage, boundary, columns, rows }
    }

    fn iter(&self) -> impl Iterator<Item=&T> {
        self.indices().map(|idx| &self[idx])
    }

    fn indices(&self) -> impl Iterator<Item=Array2DIndex> {
        struct Raw(usize, usize, usize);

        impl Iterator for Raw {
            type Item = Array2DIndex;

            fn next(&mut self) -> Option<Self::Item> {
                let Raw(value, limit, columns) = self;

                if *value < *limit {
                    let idx = *value as i32;
                    let cols = *columns as i32;
                    *value += 1;
                    Some(Array2DIndex(idx / cols, idx % cols))
                } else {
                    None
                }
            }
        }

        Raw(0, self.storage.len(), self.columns)
    }

    fn neighbours_of(&self, start: Array2DIndex) -> impl Iterator<Item=Array2DIndex> {
        static OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
        struct OffsetIter(usize, Array2DIndex);

        impl Iterator for OffsetIter {
            type Item = Array2DIndex;

            fn next(&mut self) -> Option<Self::Item> {
                if self.0 == 4 {
                    None
                } else {
                    let offset = OFFSETS[self.0];
                    self.0 += 1;
                    Some(Array2DIndex(self.1.0 + offset.0, self.1.1 + offset.1))
                }
            }
        }

        OffsetIter(0, start)
    }
}

impl<T> Index<Array2DIndex> for BoundedLinear2DArray<T> {
    type Output = T;

    fn index(&self, index: Array2DIndex) -> &Self::Output {
        let Array2DIndex(x, y) = index;

        if x < 0 || y < 0 || x as usize >= self.columns || y as usize >= self.rows {
            return &self.boundary;
        }

        &self.storage[x as usize + self.columns * y as usize]
    }
}

impl<T> IndexMut<Array2DIndex> for BoundedLinear2DArray<T> {
    fn index_mut(&mut self, index: Array2DIndex) -> &mut Self::Output {
        let Array2DIndex(x, y) = index;

        if x < 0 || y < 0 || x as usize >= self.columns || y as usize >= self.rows {
            panic!("Index ({},{}) out of range", x, y);
        }

        &mut self.storage[x as usize + self.columns * y as usize]
    }
}

fn ensure_drainage_initialized(target: &mut BoundedLinear2DArray<Drainage>, elevation: &BoundedLinear2DArray<i32>, start: Array2DIndex) {
    if target[start] == Drainage::Unknown {
        target[start] = Drainage::Pending;
        let mut drain_found = None;

        for idx in target.neighbours_of(start) {
            if elevation[idx] <= elevation[start] {
                ensure_drainage_initialized(target, elevation, idx);

                if Drainage::Ridge == target[idx] {
                    target[start] = Drainage::Ridge;
                    return;
                } else if let Drainage::DrainsTo(sink) = target[idx] {
                    if let Some(d) = drain_found {
                        if d != sink {
                            target[start] = Drainage::Ridge;
                            return;
                        }
                    } else {
                        drain_found = Some(sink)
                    }
                }
            }
        }
        if let Some(drain) = drain_found {
            target[start] = Drainage::DrainsTo(drain)
        } else {
            panic!("No drainage identified for ({},{})", start.0, start.1)
        }
    }
}

pub fn solve(input: &str) -> String {
    let line_width = input.find('\n').unwrap();
    let mut nrs = Vec::with_capacity(input.len());
    for c in input.chars() {
        if c >= '0' && c <= '9' {
            nrs.push(c as i32 - '0' as i32)
        }
    }

    let mut drainage = BoundedLinear2DArray::new(vec![Drainage::Unknown; nrs.len()], line_width, Drainage::Ridge);
    let mut total_risk = 0;
    let mut drains = Vec::new();
    let map = BoundedLinear2DArray::new(nrs, line_width, 10);

    for position in map.indices() {
        let value_here = map[position];

        if value_here == 9 {
            drainage[position] = Drainage::Ridge
        } else if map.neighbours_of(position).all(|neighbour| map[neighbour] > value_here) {
            drains.push((position, 0));
            total_risk += 1 + value_here;
            drainage[position] = Drainage::DrainsTo(position)
        }
    }

    for position in map.indices() {
        ensure_drainage_initialized(&mut drainage, &map, position);
    }

    for next in drainage.iter() {
        if let Drainage::DrainsTo(pos) = next {
            for (drain_pos, count) in &mut drains {
                if pos == drain_pos {
                    *count += 1
                }
            }
        }
    }
    let mut drain_sizes = drains.into_iter().map(|(_,count )|count).collect::<Vec<_>>();
    drain_sizes.sort();
    let best_three = drain_sizes[drain_sizes.len() - 1] *
        drain_sizes[drain_sizes.len() - 2] *
        drain_sizes[drain_sizes.len() - 3];
    format!("Total risk level: {}, Three largest basins: {}", total_risk, best_three)
}