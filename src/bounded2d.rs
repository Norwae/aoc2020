use std::ops::{Index, IndexMut};
use nom::Offset;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Array2DIndex(pub i32, pub i32);

#[derive(Debug, Clone)]
pub struct BoundedLinear2DArray<T> {
    storage: Vec<T>,
    boundary: T,
    columns: usize,
    rows: usize,
}

impl<T> BoundedLinear2DArray<T> {
    pub fn new(storage: Vec<T>, columns: usize, boundary: T) -> Self {
        let rows = storage.len() / columns;
        Self { storage, boundary, columns, rows }
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.indices().map(|idx| &self[idx])
    }

    pub fn indices(&self) -> impl Iterator<Item=Array2DIndex> {
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

    pub fn update_if_not_boundary(&mut self, index: Array2DIndex, value: T) {
        let Array2DIndex(x, y) = index;

        if !self.index_out_of_bounds(x, y) {
            let idx = self.linear_idx(x, y);
            self.storage[idx] = value
        }
    }

    pub fn diagonal_neighbours_of(&self, start: Array2DIndex) -> impl Iterator<Item=Array2DIndex> {
        static OFFSETS: [(i32, i32); 8] = [
            (-1, 0), (0, -1), (0, 1), (1, 0),
            (-1, -1), (1, -1), (1, 1), (-1, 1)
        ];
        self.neighbours(start, &OFFSETS)
    }

    pub fn direct_neighbours_of(&self, start: Array2DIndex) -> impl Iterator<Item=Array2DIndex> {
        static OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];
        self.neighbours(start, &OFFSETS)
    }

    fn neighbours(&self, start: Array2DIndex, neighbourhood: &'static [(i32, i32)]) -> impl Iterator<Item=Array2DIndex> {
        struct OffsetIter(usize, Array2DIndex, &'static [(i32, i32)]);

        impl Iterator for OffsetIter {
            type Item = Array2DIndex;

            fn next(&mut self) -> Option<Self::Item> {
                let OffsetIter(count, Array2DIndex(x, y), n) = self;
                if *count == n.len() {
                    None
                } else {
                    let (dx, dy) = n[*count];
                    *count += 1;
                    Some(Array2DIndex(*x + dx, *y + dy))
                }
            }
        }

        OffsetIter(0, start, neighbourhood)
    }


    fn linear_idx(&self, x: i32, y: i32) -> usize {
        x as usize + self.columns * y as usize
    }

    fn index_out_of_bounds(&self, x: i32, y: i32) -> bool {
        x < 0 || y < 0 || x as usize >= self.columns || y as usize >= self.rows
    }
}

impl<T> Index<Array2DIndex> for BoundedLinear2DArray<T> {
    type Output = T;

    fn index(&self, index: Array2DIndex) -> &Self::Output {
        let Array2DIndex(x, y) = index;

        if self.index_out_of_bounds(x, y) {
            return &self.boundary;
        }

        &self.storage[self.linear_idx(x, y)]
    }
}


impl<T> IndexMut<Array2DIndex> for BoundedLinear2DArray<T> {
    fn index_mut(&mut self, index: Array2DIndex) -> &mut Self::Output {
        let Array2DIndex(x, y) = index;

        if self.index_out_of_bounds(x, y) {
            panic!("Index ({},{}) out of range", x, y);
        }

        let idx = self.linear_idx(x, y);
        &mut self.storage[idx]
    }
}