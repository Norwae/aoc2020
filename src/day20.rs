use std::fmt::Display;
use std::str::FromStr;

use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, one_of};
use nom::combinator::{eof, map, verify};
use nom::lib::std::collections::{Bound, HashSet};
use nom::lib::std::fmt::Formatter;
use nom::lib::std::ops::{Index, IndexMut};
use nom::multi::many1;
use nom::sequence::{terminated, tuple};

use crate::day20::Modification::*;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Modification {
    Original,
    MirrorX,
    MirrorY,
    Rotate2,
    RotateRight,
    RotateLeft,
    RotateRightMirrorX,
    RotateLeftMirrorX,
    // rest is equivalent
}

const ALL: [Modification; 8] = [Original, MirrorX, MirrorY, Rotate2, RotateRight, RotateLeft, RotateRightMirrorX, RotateLeftMirrorX];
const FLIPPING: [Modification; 4] = [RotateLeft, RotateRight, RotateLeftMirrorX, RotateRightMirrorX];
const NO_MIRROR_FLIPPING: [Modification; 2] = [RotateLeft, RotateRight];
const NOT_FLIPPING: [Modification; 4] = [Original, MirrorX, MirrorY, Rotate2];
const NO_MIRROR_NOT_FLIPPING: [Modification; 2] = [Original, Rotate2];

trait MapFragment: Index<(usize, usize), Output=bool> {
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;

    fn for_every_index<F>(&self, mut f: F) where F: FnMut(bool, (usize, usize)) {
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                f(self[(i, j)], (i, j))
            }
        }
    }
}

#[derive(Debug)]
struct Derivation<'a> {
    tile: &'a Tile,
    modifier: Modification,
}

impl Index<(usize, usize)> for Derivation<'_> {
    type Output = bool;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        let actual_idx = match self.modifier {
            Original => (row, col),
            MirrorX => (row, (self.tile.cols - col) - 1),
            MirrorY => ((self.tile.rows - row) - 1, col),
            Rotate2 => ((self.tile.rows - row) - 1, (self.tile.cols - col) - 1),
            RotateLeft => (col, (self.tile.cols - row) - 1),
            RotateRight => ((self.tile.rows - col) - 1, row),
            RotateRightMirrorX => (col, row),
            RotateLeftMirrorX => ((self.tile.rows - col) - 1, (self.tile.cols - row) - 1)
        };

        &self.tile[actual_idx]
    }
}

impl MapFragment for Derivation<'_> {
    fn rows(&self) -> usize {
        match self.modifier {
            Original | MirrorY | MirrorX | Rotate2 => self.tile.rows,
            _ => self.tile.cols
        }
    }

    fn cols(&self) -> usize {
        match self.modifier {
            Original | MirrorY | MirrorX | Rotate2 => self.tile.cols,
            _ => self.tile.rows
        }
    }
}

#[derive(Debug, Eq, Clone)]
struct Tile {
    codes: Vec<u32>,
    contents: Vec<bool>,
    rows: usize,
    cols: usize,
}

impl MapFragment for Tile {
    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        if other.codes != self.codes {
            return false;
        }

        symmetric_equivalent(self, other)
    }
}

impl Index<(usize, usize)> for Tile {
    type Output = bool;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.contents[row * self.cols + col]
    }
}

impl IndexMut<(usize, usize)> for Tile {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.contents[row * self.cols + col]
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Combined Tile {:?}:\n", self.codes))?;
        for row in 0..self.rows {
            for col in 0..self.cols {
                let next_char = if self[(row, col)] { "#" } else { "." };
                f.write_str(next_char)?;
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct TileView<'a, F> {
    tile: &'a F,
    row_offset: usize,
    col_offset: usize,
    rows: usize,
    cols: usize,
}

impl<F: MapFragment> Index<(usize, usize)> for TileView<'_, F> {
    type Output = bool;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        let row = row + self.row_offset;
        let col = col + self.col_offset;
        &self.tile[(row, col)]
    }
}

impl<F: MapFragment> MapFragment for TileView<'_, F> {
    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }
}

impl<'a, T: MapFragment> TileView<'a, T> {
    fn new(tile: &'a T, row_offset: usize, col_offset: usize, rows: usize, cols: usize) -> Self {
        assert!(rows <= tile.rows() - row_offset);
        assert!(cols <= tile.cols() - col_offset);
        Self { rows, cols, row_offset, col_offset, tile }
    }
}

fn symmetric_equivalent<M: MapFragment>(mine: &M, other: &Tile) -> bool {
    let mut slicer = 4..4;
    if mine.rows() == other.rows && mine.cols() == other.cols {
        slicer.start = 0;
    }

    if mine.cols() == other.rows && mine.rows() == other.cols {
        slicer.end = 8;
    }

    'modification: for m in &ALL[slicer] {
        let other = other.modify(*m);
        let mut mismatched = false;

        mine.for_every_index(|my_value, idx| mismatched |= my_value != other[idx]);
        if !mismatched {
            return true;
        }
    }

    false
}


fn serpent_tiles<T: MapFragment>(tile: &T) -> Vec<(usize, usize)> {
    /*
     01234567890123456789
   0 ..................#.
   1 #....##....##....###
   2 .#..#..#..#..#..#...
    */
    const SERPENT_CHECK_OFFSETS: [(usize, usize); 15] =
        [
            (0, 18),
            (1, 0), (1, 5), (1, 6), (1, 11), (1, 12), (1, 17), (1, 18), (1, 19),
            (2, 1), (2, 4), (2, 7), (2, 10), (2, 13), (2, 16)
        ];
    let mut all_tiles = HashSet::new();

    for start_row in 0..=tile.rows() - 3 {
        'next_offset: for start_col in 0..=tile.cols() - 20 {
            let view = TileView::new(tile, start_row, start_col, 3, 20);
            for offset in &SERPENT_CHECK_OFFSETS {
                if !view[*offset] {
                    continue 'next_offset;
                }
            }

            for (row, col) in &SERPENT_CHECK_OFFSETS {
                all_tiles.insert((row + start_row, col + start_col));
            }
        }
    }

    let mut result = all_tiles.into_iter().collect::<Vec<_>>();
    result.sort();
    result
}


impl Tile {
    fn modify(&self, modifier: Modification) -> Derivation {
        Derivation { tile: self, modifier }
    }

    fn merged_with(&self, other: &Self) -> Vec<Self> {
        let mut target = Vec::new();
        for code in &self.codes {
            if other.codes.contains(code) {
                return target;
            }
        }

        if self.cols == other.cols {
            self.merge_at_top_row(other, &NO_MIRROR_NOT_FLIPPING[..], &NOT_FLIPPING, &mut target);
        }

        if self.rows == other.rows {
            self.merge_at_top_row(other, &NO_MIRROR_FLIPPING[..], &FLIPPING[..], &mut target);
        }

        if self.rows == other.cols {
            self.merge_at_top_row(other, &NO_MIRROR_FLIPPING[..], &NOT_FLIPPING[..], &mut target);
        }

        if self.cols == other.rows {
            self.merge_at_top_row(other, &NO_MIRROR_NOT_FLIPPING[..], &FLIPPING[..], &mut target);
        }

        deduplicate(target)
    }

    fn merge_at_top_row(self: &Tile, t2: &Tile, mods1: &[Modification], mods2: &[Modification], target: &mut Vec<Tile>) {
        for m1 in mods1 {
            for m2 in mods2 {
                let upper = self.modify(*m1);
                let lower = t2.modify(*m2);
                let cols = upper.cols();
                assert_eq!(upper.cols(), lower.cols());
                let bottom_row_upper = TileView::new(&upper, upper.rows() - 1, 0, 1, cols);
                let top_row_lower = TileView::new(&lower, 0, 0, 1, cols);

                if (0..cols).all(|idx| bottom_row_upper[(0, idx)] ==
                    top_row_lower[(0, idx)]) {
                    let mut codes = self.codes.clone();
                    codes.extend_from_slice(t2.codes.as_slice());
                    codes.sort();
                    let rows = upper.rows() + lower.rows();
                    let cols = upper.cols();
                    let mut contents = Vec::with_capacity(rows * cols);

                    upper.for_every_index(|v, _| contents.push(v));
                    lower.for_every_index(|v, _| contents.push(v));

                    target.push(Self { codes, rows, cols, contents })
                }
            }
        }
    }
}


fn deduplicate<T: Eq>(tiles: Vec<T>) -> Vec<T> {
    tiles.into_iter().fold(
        Vec::new(),
        |mut v, next| {
            if !v.contains(&next) {
                v.push(next);
            }
            v
        },
    )
}

fn parse(input: &str) -> IResult<&str, Tile> {
    map(tuple((
        tag("Tile "),
        digit1,
        tag(":\n"),
        many1(
            terminated(
                many1(
                    map(
                        one_of(".#"),
                        |c| c == '#',
                    )
                ),
                tag("\n"),
            )
        ),
        alt((tag("\n"), eof))
    )), |(_, nr, _, lines, _)| {
        let rows = lines.len();
        let cols = lines[0].len();
        let contents = lines.concat();
        let codes = vec![u32::from_str(nr).unwrap()];
        Tile { rows, cols, contents, codes }
    })(input)
}

pub fn solve() {
    let (_, single_tiles) = many1(parse)(INPUT).unwrap();
    let tile_combos_2 = full_combination(&single_tiles, &single_tiles);
    let tile_combos_4 = full_combination(&tile_combos_2, &tile_combos_2);
    let tile_combos_8 = full_combination(&tile_combos_4, &tile_combos_4);
    let tile_combos_12 = full_combination(&tile_combos_8, &tile_combos_4);
    let tile_combos_24 = full_combination(&tile_combos_12, &tile_combos_12);
    let tile_combos_48 = full_combination(&tile_combos_24, &tile_combos_24);
    let tile_combos_72 = full_combination(&tile_combos_48, &tile_combos_24);

    let full_combo = full_combination(&tile_combos_72, &tile_combos_72);
    let solution = full_combo.first().unwrap().clone();
    let mut checksum: u64 = 1;

    for m in &[Original, RotateRight, Rotate2, RotateLeft] {
        let solution = solution.modify(*m);
        let solution = TileView::new(&solution, 0, 0, 10, 10);

        checksum *= single_tiles.iter().find_map(|x| if symmetric_equivalent(&solution, x) { Some(x.codes[0]) } else { None }).unwrap() as u64;
    }

    println!("Image checksum is {}", checksum);
    let sea_monster_map = remove_borders(solution);

    assert_eq!(sea_monster_map.contents.len(), sea_monster_map.cols * sea_monster_map.rows);

    let potential_roughs = sea_monster_map.contents.iter().filter(|x| **x).count();
    println!("There are overall {} potential rough tiles", potential_roughs);
    for m in &ALL {
        let modified = sea_monster_map.modify(*m);
        let exclude = serpent_tiles(&modified).len();
        println!("When viewed with modification {:?}, there are {} rough tiles", m, potential_roughs - exclude);
    }
    println!("{}", sea_monster_map)
}

fn remove_borders(solution: Tile) -> Tile {
    let rows = solution.rows - (2 * solution.rows / 10);
    let cols = solution.cols - (2 * solution.cols / 10);
    let mut sea_monster_map = Tile {
        codes: vec![0],
        rows,
        cols,
        contents: Vec::with_capacity(rows * cols),
    };

    solution.for_every_index(|v, (row, col)| {
        let row_discriminant = row % 10;
        let col_discriminant = col % 10;

        if row_discriminant != 0 && row_discriminant != 9 && col_discriminant != 0 && col_discriminant != 9 {
            sea_monster_map.contents.push(v)
        }
    });

    sea_monster_map
}

fn full_combination(seed_tiles_1: &Vec<Tile>, seed_tiles_2: &Vec<Tile>) -> Vec<Tile> {
    let mut temp = Vec::new();
    for seed_left in seed_tiles_1 {
        for seed_right in seed_tiles_2 {
            temp.append(&mut seed_left.merged_with(seed_right));
        }
    }
    let generated_tiles = deduplicate(temp);
    println!("Generated {} candidates (size {}) from {}x{} seeds",
             generated_tiles.len(),
             seed_tiles_1.first().unwrap().contents.len() *
                 seed_tiles_2.first().unwrap().contents.len(),
             seed_tiles_1.len(),
             seed_tiles_2.len());
    generated_tiles
}

#[cfg(test)]
mod test {
    use crate::day20::*;

    fn must_parse(input: &str) -> Tile {
        parse(input).unwrap().1
    }

    fn must_parse_combined(sources: &[u32], input: &str) -> Tile {
        Tile {
            codes: sources.to_vec(),
            ..must_parse(input)
        }
    }

    const CANONICAL_SERPENT: &str = "Tile 9999:
..................#.
#....##....##....###
.#..#..#..#..#..#...
";/*
01234567890123456789 */

    #[test]
    fn trim_borders() {
        let original = must_parse("Tile 0:
#...##.#....###..####.#.#####.
..#.#..#.####...#.#..#..######
.###....#...#....#....#.......
###.##.##..#.#.#..########....
.###.#######...#.#######.#..#.
.##.#....###.##.###..#...#.##.
#...##########.#...##.#####.##
.....#..###...##..#...#.###...
#.####...###..#.......#.......
#.##...##...##.#..#...#.###...
#.##...##...##.#..#...#.###...
##..#.##....#..###.###.##....#
##.####....#.####.#...#.###..#
####.#.#.....#.########.#..###
.#.####......##..##..######.##
.##..##.#.....#...###.#.#.#...
....#..#.##.#.#.##.##.###.###.
..#.#......#.##.#..##.###.##..
####.#.....#..#.##...######...
...#.#.#.####.##.#...##...####
...#.#.#.####.##.#...##...####
..#.#.###...##.##.###..#.##..#
..####.#####.#...##..#.#..#.##
#..#.#..#....#.#.#...####.###.
.#..####.##..#.#.#.#####.###..
.#####..#######...#..##....##.
##.##..#....#...#....####...#.
#.#.###....##..##....####.##.#
#...###.....##...#.....#..####
..#.#....###.#.#.......##.....
");
        let slimmed = remove_borders(original);
        assert_eq!(slimmed, must_parse("Tile 0:
.#.#..#.##...#.##..#####
###....#.#....#..#......
##.##.###.#.#..######...
###.#####...#.#####.#..#
##.#....#.##.####...#.##
...########.#....#####.#
....#..#...##..#.#.###..
.####...#..#.....#......
#..#.##..#..###.#.##....
#.####..#.####.#.#.###..
###.#.#...#.######.#..##
#.####....##..########.#
##..##.#...#...#.#.#.#..
...#..#..#.#.##..###.###
.#.#....#.##.#...###.##.
###.#...#..#.##.######..
.#.#.###.##.##.#..#.##..
.####.###.#...###.#..#.#
..#.#..#..#.#.#.####.###
#..####...#.#.#.###.###.
#####..#####...###....##
#.##..#..#...#..####...#
.#.###..##..##..####.##.
...###...##...#...#..###
"))
    }

    #[test]
    fn find_sea_serpent() {
        let canonical_serpent = must_parse(CANONICAL_SERPENT);
        let serpent_indices = serpent_tiles(&canonical_serpent);
        assert_eq!(serpent_indices, vec![
            (0, 18),
            (1, 0), (1, 5), (1, 6), (1, 11), (1, 12), (1, 17), (1, 18), (1, 19),
            (2, 1), (2, 4), (2, 7), (2, 10), (2, 13), (2, 16),
        ]);
    }

    #[test]
    fn find_two_serpents() {
        let example = must_parse("Tile 9999:
.####...#####..#...###..
#####..#..#.#.####..#.#.
.#.#...#.###...#.##.##..
#.#.##.###.#.##.##.#####
..##.###.####..#.####.##
...#.#..##.##...#..#..##
#.##.#..#.#..#..##.#.#..
.###.##.....#...###.#...
#.####.#.#....##.#..#.#.
##...#..#....#..#...####
..#.##...###..#.#####..#
....#.##.#.#####....#...
..##.##.###.....#.##..#.
#...#...###..####....##.
.#.##...#.##.#.#.###...#
#.###.#..####...##..#...
#.###...#.##...#.######.
.###.###.#######..#####.
..##.#..#..#.#######.###
#.#..##.########..#..##.
#.#####..#.#...##..#....
#....##..#.#########..##
#...#.....#..##...###.##
#..###....##.#...##.##.#
");
        let serpent_indices = serpent_tiles(&example);
        assert_eq!(serpent_indices.len(), 30);
    }

    #[test]
    fn parse_a_tile() {
        let (rest, parsed_tile) = parse("Tile 3:
####
##.#
#..#
.##.
").unwrap();
        assert_eq!("", rest);
        assert_eq!(parsed_tile, Tile {
            codes: vec![3],
            contents: vec![
                true, true, true, true,
                true, true, false, true,
                true, false, false, true,
                false, true, true, false
            ],
            rows: 4,
            cols: 4,
        })
    }

    #[test]
    fn modify_original() {
        let orig = must_parse("Tile 848:
#...#
.#...
..#..
");
        assert!(compare_only_contents(orig.modify(Modification::Original), orig.clone()))
    }

    #[test]
    fn modify_mirror_x() {
        let orig = must_parse("Tile 848:
#...#
.#...
..#..
");
        let rotated = must_parse("Tile 848:
#...#
...#.
..#..
");
        assert!(compare_only_contents(orig.modify(Modification::MirrorX), rotated.clone()))
    }

    #[test]
    fn modify_mirror_y() {
        let orig = must_parse("Tile 848:
#...#
.#...
..#..
");
        let rotated = must_parse("Tile 848:
..#..
.#...
#...#
");
        assert!(compare_only_contents(orig.modify(Modification::MirrorY), rotated))
    }


    #[test]
    fn modify_rotate_right() {
        let orig = must_parse("Tile 848:
#...#
.#...
..#..
");
        let rotated = must_parse("Tile 848:
..#
.#.
#..
...
..#
");
        assert!(compare_only_contents(orig.modify(Modification::RotateRight), rotated))
    }


    #[test]
    fn modify_rotate_right_mirror_x() {
        let orig = must_parse("Tile 848:
#...#
.#...
..#..
");
        let rotated = must_parse("Tile 848:
#..
.#.
..#
...
#..
");
        assert!(compare_only_contents(orig.modify(Modification::RotateRightMirrorX), rotated))
    }

    #[test]
    fn modify_rotate_2() {
        let orig = must_parse("Tile 848:
#...#
.#...
..#..
");
        let rotated = must_parse("Tile 848:
..#..
...#.
#...#
");
        assert!(compare_only_contents(orig.modify(Modification::Rotate2), rotated))
    }

    #[test]
    fn modify_rotate_left() {
        let orig = must_parse("Tile 848:
#...#
.#...
..#..
");
        let rotated = must_parse("Tile 848:
#..
...
..#
.#.
#..
");
        assert!(compare_only_contents(orig.modify(Modification::RotateLeft), rotated))
    }

    #[test]
    fn modify_rotate_left_mirror_x() {
        let orig = must_parse("Tile 848:
#...#
.#...
..#..
");
        let rotated = must_parse("Tile 848:
..#
...
#..
.#.
..#
");
        assert!(compare_only_contents(orig.modify(RotateLeftMirrorX), rotated))
    }

    fn compare_only_contents<T1: MapFragment, T2: MapFragment>(lhs: T1, rhs: T2) -> bool {
        let mut matched = true;
        lhs.for_every_index(|v,idx|{
            matched &= v == rhs[idx]
        });

        matched
    }

    #[test]
    fn merge_mismatched_sizes() {
        let left = must_parse("Tile 1:
#..
#..
.##
");
        let right = must_parse("Tile 2:
.##.
#..#
#..#
.##.
");
        assert!(left.merged_with(&right).is_empty());
    }

    #[test]
    fn merge_compatibility_not_okay() {
        let left = must_parse("Tile 1:
.#.#
#.#.
.#.#
#.#.
");
        let right = must_parse("Tile 2:
.##.
#..#
#..#
.##.
");
        assert!(left.merged_with(&right).is_empty());
    }

    #[test]
    fn merge_compatible_simple() {
        let left = must_parse("Tile 1:
.#.#
#.#.
.#.#
#.#.
");
        let right = must_parse("Tile 2:
#.#.
####
####
####
");
        let merged = left.merged_with(&right);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0], Tile {
            codes: vec![1, 2],
            ..must_parse("Tile 1:
.#.#
#.#.
.#.#
#.#.
#.#.
####
####
####
")
        });
    }

    #[test]
    fn merge_compatible_non_square() {
        let left = must_parse("Tile 1:
##..#
.....
#.##.
");
        let right = must_parse("Tile 2:
##
#.
.#
..
#.
");
        let result = left.merged_with(&right);
        assert_eq!(result.len(), 1);
        println!("{}", &result[0]);
        assert!(result.contains(&must_parse_combined(&[1, 2], "Tile 1:
#.##.
.....
##..#
##..#
#.#..
",
        )));
    }

    #[test]
    fn merge_multiple_steps() {
        let part1 = must_parse("Tile 1:
###
##.
.#.
");
        let part2 = must_parse("Tile 2:
..
.#
##
");
        let part3 = must_parse("Tile 3:
#####
");
        let merged_complete = part1.merged_with(&part2).iter().flat_map(|partial| partial.merged_with(&part3).into_iter()).collect::<Vec<_>>();

        assert!(merged_complete.len() > 0);
        assert!(merged_complete.contains(&must_parse_combined(&[1, 2, 3], "Tile 9:
.#...
.###.
#####
#####
")));
    }
}

pub const EXAMPLE_INPUT: &str = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
";

pub const INPUT: &str = "Tile 1249:
...#......
#..#..#.##
##........
#.#.......
..........
#...###...
#..#......
#...##....
..........
.....#...#

Tile 1693:
..#..####.
#.........
##..#....#
#.....#..#
......#.##
#........#
.....##...
.#..##.#.#
##........
###..#....

Tile 1481:
....#####.
#....#..##
##..#....#
##..#...#.
#..#.#...#
..........
...#..#...
.#...#...#
#...#.....
..#..##.##

Tile 3169:
####...#.#
#.#.....#.
.#......##
..#.#....#
...###..#.
#....###.#
.......#..
##.##.....
.#.#......
.#....#...

Tile 1229:
##..###..#
#.......#.
..#..##..#
##..#.....
#.#..#..#.
.#..#.#.##
....#....#
#..#..#.##
....#....#
..##..####

Tile 1489:
#......#..
....#.....
#.....#..#
#.......#.
#.#..#..#.
#.........
#........#
#..#.#....
#.........
....####.#

Tile 2477:
#####.##.#
.###...###
#....#..##
.#.#..#..#
###.##...#
#........#
#..#..#...
.......#.#
#......###
##.##.##.#

Tile 2897:
##.##..#.#
#.......##
#.#..#.#..
..#...#..#
...##.#..#
..#.......
#.#..##..#
..#....#.#
#....#.#.#
#......###

Tile 2083:
..#...##.#
###.#.##..
....##....
#...#..#..
...##....#
#..#......
.#.##.....
..##..####
....###..#
.#...#.#..

Tile 1069:
..#.#.#..#
..#....#.#
.........#
##..#.....
#.....#...
..##......
#..#......
.##..#####
#.#....#..
.....#..#.

Tile 1427:
...####.##
.###......
.#..#.#..#
..#.###.##
.#..#.....
..##.#....
.#......#.
#....#...#
.......#..
#.#..###..

Tile 1429:
.##.#.#.#.
#..##....#
..#......#
...#....##
...#.##..#
..#.#.....
#....#..##
#..#.....#
.##....#..
##.#.#...#

Tile 2357:
#.##..#.##
.........#
#..#.#..##
#....#.#..
#........#
#...##....
#....#....
....##...#
#.#..##...
.###.#.#..

Tile 3181:
...#..####
........#.
#...#...##
#.#.....##
#.........
##...#....
#.##.....#
#....#...#
..#.#...##
#..###.##.

Tile 2887:
.#..#.###.
#.........
.#....#...
#........#
..#.##...#
.......##.
....#.#..#
#...###..#
.#...#..##
###...#..#

Tile 2837:
##.##..###
.......#.#
....#.#..#
.......##.
.....#....
#.#.##..#.
#.#..#...#
....#.##..
..#......#
...######.

Tile 2539:
.###....#.
.......##.
##.##...#.
.###.....#
###..##...
.##....#.#
........#.
..........
.#........
#.....#...

Tile 2399:
##.#..##..
#...#.#..#
.##......#
###..##...
.#....#...
....#.#...
.....###.#
#.......##
#......#..
.##.##....

Tile 2383:
#..##.###.
.#.......#
#.#..#....
#..#.#...#
#.#.#.....
......#...
#....#..#.
..#.#.#..#
#..#.##..#
.#.##...##

Tile 2521:
.#..#.##.#
#........#
#.#...#.##
#........#
##.#.#..##
#..######.
.##..##..#
.#.....##.
.#.#......
..#..#####

Tile 1823:
#.#...##..
#..#.....#
.##......#
#...###.#.
......#...
.....##...
#....##.#.
#.........
#..#.....#
#.#.##.#..

Tile 1301:
####.#.###
#.##...##.
..#...####
..........
#....#..##
....#..#..
#..#.#.#..
...#...#..
###..###.#
.##..#.#.#

Tile 1289:
#.###....#
#.#.....#.
#.....##.#
#........#
.##.#...#.
.###..#...
..#.......
.##...#...
...#....##
.###....#.

Tile 3823:
#.##.##..#
.........#
#...##.#.#
.#.##.....
##.....#..
.#..#....#
#...#...##
#.........
#........#
.#.##.####

Tile 2411:
#.####.###
#....#.###
..##.....#
#.#...#..#
##...#.#.#
##.##...##
#.......#.
#...##...#
.........#
###.##.##.

Tile 1039:
.......#.#
..#.......
........#.
.##...####
##...#..##
.#.......#
..#.#.....
...#.....#
.........#
..#...#...

Tile 1609:
.##..#..##
##.###.###
.#.....#..
#........#
...#...#..
#.......##
...#..###.
#.##.##.##
#.......#.
...#.#....

Tile 2017:
##..#..#..
....###...
..##......
#...#.###.
#....#....
#.#.......
#..#.#....
###..##..#
###.#....#
#.#.#.#.##

Tile 3301:
.#####...#
...##.....
#..###...#
#....#...#
...#.#...#
#......#.#
#........#
...#......
.......#.#
..#####...

Tile 3733:
#####.##.#
#.##..#..#
#....#..##
#....#....
.....##...
#......#..
#....##.##
.........#
.#..#....#
.....##...

Tile 2309:
#..##.##..
...#.#.#.#
#..##.#..#
....##....
##....#..#
##..##.#.#
.###....#.
###...#.##
....#.....
.#.#.###.#

Tile 2879:
#.##.##...
...#.....#
...##.####
...#.#.#.#
#.###.##.#
.....#...#
#.....#.#.
#..#.#..##
.....###..
.##..#.###

Tile 3583:
.#.##...#.
..##...##.
#.#....##.
#.##...#.#
.....#..##
#####.....
..###..#..
..##..#.#.
.##....##.
##.##...#.

Tile 2153:
##...####.
..#.#...#.
##..##..#.
.#..#.#...
..........
..#...#...
#....#.###
...##....#
#.##.....#
#...#....#

Tile 3581:
#...##.###
..####..##
...##....#
.#..#.....
.##......#
#.#......#
...###.#.#
###.#.....
#.##.##...
#.#.....#.

Tile 2927:
##.#.#..##
..........
..........
.....#....
#..#..#..#
....#.#...
.#...#...#
.#...##..#
......#...
###.#.##.#

Tile 2861:
.#..#.###.
#..#....##
#.#.#..#..
........#.
..#.#.#...
...##....#
#..###.#.#
#..#..##..
..#.#....#
.#..####.#

Tile 2851:
#.#....#..
#........#
.#........
#..##.#...
..##....##
.#.......#
#.........
......#..#
..........
###.##..##

Tile 3319:
...##....#
#.......##
##.#..#...
.##....###
#...#...##
..#.......
.###..#..#
#...#.....
.........#
.#..##.#..

Tile 2143:
...#......
##.#...#.#
##.......#
#.##..#..#
..#...#..#
.#.#...#..
..#....#..
......#...
#.#.......
#..#..#.##

Tile 1093:
#.#...#.#.
##..#.....
.........#
##..#...##
##..#..#..
##..##....
##.##....#
#.#....#..
.#.#..#..#
..#.##.###

Tile 3391:
#....#..#.
#...#..#..
##....##..
..#.#..#.#
#####...##
..#..#...#
#.#......#
.#...#....
.....##.##
.#.#.#....

Tile 3917:
...##.#...
#..#.....#
#..##.###.
#.#..#..#.
.####..#.#
..##.#.###
#.#..##.#.
#....##..#
..###.#..#
..##.####.

Tile 1847:
#.##.#..#.
.#.##.....
#.#..#..#.
#....#.#..
.#.......#
..#..#.#.#
..##....##
#.#.##....
##.#..#...
.##.##.###

Tile 1667:
.##..#..#.
###....##.
#.####.#.#
...#.....#
.#.#..##.#
.#..#....#
#.##.#...#
#.#.#.....
...###...#
#.#..####.

Tile 1217:
...#####..
....#..#.#
..#...####
#........#
#........#
###..#...#
.##.##...#
.#.......#
.....##...
.##.#.###.

Tile 3467:
###.##....
.......#.#
.....#..##
##..#....#
....#..#..
....#..###
#......###
#.##...#..
..#.....##
...#.#..#.

Tile 1297:
..###..##.
#....#.#.#
.........#
###.#....#
#......#.#
##....##..
#.#.##..##
.##.....#.
.....#...#
.#.##.....

Tile 3877:
..###.#...
##..#....#
###.##..##
.#..######
....#.#..#
.#.......#
#...##.#..
..#..#....
#.##.##..#
#..##.#.#.

Tile 2389:
##...#####
##..#.#..#
#..#.....#
......#..#
.#........
#....##.#.
.##....#..
#.....#.#.
#........#
...##.#...

Tile 3361:
.....#.#..
.........#
...#....#.
##.##.....
#....##...
##...#...#
#...####.#
..##.....#
...#...#.#
###.####..

Tile 2207:
###.##..#.
#..###....
......#..#
#........#
##....##.#
........#.
...#..#...
#....#..##
...####...
##........

Tile 1997:
..#.#..###
......#...
...#...#..
..#......#
.........#
#....#...#
....#..#..
.....#....
#........#
#....#####

Tile 1063:
#.#...##..
.##..#....
#.#..##...
..#...####
......##..
#...#.#...
####......
.#.....#..
.#.......#
#...####..

Tile 3109:
...###.###
.......#..
......##.#
.#......##
#..##..#..
##..#..#.#
.....##.##
#....#.#.#
.....###..
####....#.

Tile 1097:
#####.##..
.#...#.##.
..#..#....
#..#....#.
#.#....#.#
.#...#.#.#
#....#..##
#....#...#
...#...#..
...#.#..#.

Tile 1117:
#..##.#.##
#.#.....#.
...#.#...#
#.#..##.#.
#.###..#..
#....#...#
....#.....
.#...#...#
..##..#..#
.#.#.....#

Tile 2551:
.#..#.####
#...#.....
##....##.#
#....#....
.##.....#.
.#...#....
#....#.#..
......#...
##.#.#....
.#....#..#

Tile 2677:
.#.####..#
#.#.....##
...#.#....
.#...#...#
.##...#..#
##....#...
...##.#..#
##...#..##
##...#...#
##...###.#

Tile 1367:
.#.##...##
#.#..#....
....##.#..
.........#
.#....##.#
....#.##..
....##...#
#.#..#...#
#.#.#..##.
....#.##.#

Tile 1913:
.##..#..##
.#...##.##
..#.....#.
......#...
...#...##.
...####...
..#..#....
....#.#...
#.........
.####.####

Tile 1709:
.#....##..
..#...#.##
.#......#.
...#....##
.#.##...#.
.#.#.#.###
.........#
#.......#.
##....#...
..#.##.###

Tile 1459:
.##..#.#..
...##.....
.#.#...###
...#.....#
##..#..#.#
.........#
.........#
....#...#.
#..#......
...#.#...#

Tile 2137:
##..#....#
#.#####.#.
......#..#
#.#......#
#...#..#.#
#..#..#.##
###..####.
.##..#...#
#.#.......
.##.##....

Tile 2659:
#.##..#...
#...##....
#...#...#.
#........#
#....#....
.......##.
..#.....#.
#.#..##..#
#......#..
..###.#..#

Tile 2657:
####...##.
........#.
..####..#.
..#.......
....#.#..#
#.#.#.#...
...#....#.
###...####
........##
##.#.#.#..

Tile 2099:
#.###.##.#
#..#...###
..#......#
.#...#.#..
...##.....
###.##...#
..##..#...
...#......
..#.#.#.#.
..#.##....

Tile 3209:
##.###....
...#....#.
#..##..##.
##.##.#...
.#..#....#
#..#.#...#
.#...#...#
#..#......
...#......
..#.#.###.

Tile 1879:
#.##.#.##.
.#.##.#...
#..#.#....
#.....###.
#..##.....
....##....
#..#..#...
#.........
..#.##....
...####.##

Tile 1621:
###....#.#
.........#
.#....##..
.....#.#..
.#...#.#..
##...####.
.....#...#
#.........
....#.#..#
.####...##

Tile 3931:
.####.###.
#.##.....#
#..#.#...#
..#......#
###..#.##.
....##....
#..#......
..........
.#.....#.#
#.##.#..##

Tile 2777:
##....####
.#.#......
##......#.
#.......##
.#..#.#..#
##.#...#.#
##.#.....#
...##....#
#..#.#....
#.#..#....

Tile 2909:
#......##.
#........#
..#.#.###.
.......#.#
........##
....#.....
......#...
#........#
#.......#.
......#.#.

Tile 1777:
.##...####
.##.....#.
#..#.#..#.
.#......##
.#....#..#
#.###.#...
.#.#......
###..#...#
.#.......#
..#...#.##

Tile 3251:
...#.###..
#....##..#
#....#..#.
#....#..##
.....#...#
#.......##
.......#.#
#....#..#.
#...##...#
.#.#.....#

Tile 1601:
#....#....
.##.##...#
.........#
#.........
.#.....###
#..##.#.#.
#......#.#
.#..#.#..#
.#...#....
.#.#.#..##

Tile 1283:
#####.#..#
..#....##.
#..#......
.#....##..
.#.#.....#
##......##
#.....##..
##..#....#
#........#
.#.###..##

Tile 3079:
..#.##....
.......#.#
..#...##.#
..###...#.
#..#.....#
..##.....#
..#..#.#..
.##.#.#..#
#..#......
.#.##..#..

Tile 3793:
##..#####.
##...#....
##....#...
..........
#.........
.#....#..#
#....#..#.
#....#....
###..##...
#.###.##..

Tile 3037:
.##.#..##.
#..#.###..
.#....#...
...####..#
..#.###..#
#.#..#....
.##......#
#.....#..#
#...#.#...
##.######.

Tile 1669:
##.#......
.#......#.
##.#...#.#
#.#.#.#..#
##........
##....#...
#.#.....#.
..........
.........#
####...###

Tile 1087:
.###.#.#.#
###.......
.....#...#
##.....#..
.#.#......
...#......
#...#..#.#
#........#
#..#....##
.#...#####

Tile 2617:
.##..#.#..
.....#...#
..#.#.....
....#...##
####...##.
..##......
...#...#..
..#.#.##.#
#.#.......
###.##....

Tile 3943:
.#.####..#
.........#
#.........
.#........
###..#....
...#....#.
#...#.#...
.#......#.
...##.#...
..##....#.

Tile 2273:
##.#.#...#
...##.#..#
..##......
..#.....#.
#......#..
...#....#.
.##....#.#
...#.#.#..
#..###.#.#
###.#..##.

Tile 3803:
##....#..#
##...#.#.#
..##....#.
#...#..###
....#.....
....#.####
#.....##..
##....###.
#....#..##
#####.##..

Tile 1697:
....####.#
#....#.#.#
#.....#.##
#......#..
#.#...#...
#....#.#..
.##....#..
##...#.#.#
#.#..#..#.
.......#..

Tile 3343:
######.#..
#....##...
#.#.##...#
#...####..
#......#..
##.#..##.#
#.....##.#
..##.##..#
..#......#
###..##...

Tile 1549:
#.##.###.#
.....###..
...#.###..
##...##..#
#.#.......
#.........
....#....#
..#.....##
.....##...
#.#.##.##.

Tile 1619:
..###.###.
##.###...#
#...##...#
....##...#
..##..#.#.
#...#.#..#
#.......##
#..#.....#
....##...#
.#...#.#..

Tile 2971:
..##.###..
#.....#..#
#..#.....#
#..#...#.#
...#......
#......###
#....####.
.#....#...
...#.....#
#...###..#

Tile 3617:
####...#..
.......#.#
.#.#......
#..##..###
.##.......
.#....#..#
.....#.###
....##.#..
#....#...#
.#.##.####

Tile 1543:
......#.##
#......#.#
##.....###
#...#...##
#.#....#..
.#........
#....#....
......##..
##...#..#.
#...#.##.#

Tile 2111:
####..#...
#..#...#.#
#.#.#.##..
##........
#...#..#..
#.##..#...
#.........
##.#..#...
.##.#.###.
#.##.####.

Tile 1499:
#.####.#..
..##.#...#
#..#......
....#.....
.#..##...#
#.#...##.#
...#.##.##
###..#.###
.....#....
...##.##.#

Tile 3461:
.#####.#.#
#.........
..#..#....
..#......#
###...####
...#.##..#
.#........
#..#....##
#...#...##
####.....#

Tile 1483:
.....#.##.
##........
#.#.....#.
##......#.
#.....#...
..#.......
##....#..#
##........
..#...####
......###.

Tile 2467:
###.#.#...
#.#.....#.
#...#....#
#....#...#
...#..#..#
##...#....
#.#.###...
#..##..#.#
#.........
..######.#

Tile 3631:
###.#...##
.........#
#.##..#...
...##..#..
##..#.#..#
#..#.#....
......#.#.
..........
#......##.
..##.#....

Tile 3767:
#...#.#..#
..........
#......#.#
#...#.#..#
##.......#
..#......#
........#.
##...##.##
#........#
....#....#

Tile 2381:
.#.#.##...
#.##.#....
...##.#.##
#.....#.#.
#......#..
...#....#.
......####
#.#.#.....
##...##..#
...#...#.#

Tile 2687:
###...####
.....#.###
..#.#.#.#.
##..#...##
#.###...#.
..#.##..##
##.....#..
##..#..#.#
#...#..#.#
#.#..###.#

Tile 3719:
..####.#..
.#......##
#......###
##....###.
.#...##.##
.#.....#.#
..##..#.#.
#......###
...#...#..
......###.

Tile 3259:
######.#.#
#..#......
.........#
#.......#.
..##...#..
##...##.##
#......#.#
##.......#
#.......#.
#..##.#.##

Tile 3643:
###..#..##
#.#.#.#...
..##....#.
##....##..
..##......
#..#..#.##
......##..
#.......#.
#...##..##
#.#..#.###

Tile 2767:
####....##
..........
.........#
..#....#.#
#......###
#.##..#..#
#.#..#.#..
...#.#...#
#..####..#
##..#####.

Tile 2333:
###..#.##.
..##.....#
..#.#..##.
#.....#..#
....#.#...
##...#....
#.#......#
###.......
#...#....#
..#..#..##

Tile 2857:
..#..##..#
#.....#...
##....#..#
#...##....
....#.#..#
#.#.##....
#...##..#.
.#...##..#
#...#.#...
.#.###.#.#

Tile 1193:
#.#..#.#.#
##..####..
###..##.#.
#.......##
###....##.
...#...#..
#...#....#
##.......#
#####.##.#
..#####.##

Tile 2351:
.##..#####
#.........
.#..#.....
#.##.#...#
..#.#..#..
#.###....#
###..#....
##.......#
#...#..#..
.....#...#

Tile 1129:
..##..####
.....##..#
##........
.........#
#...#.....
....#...##
..#....##.
..#.....#.
..#.......
.##.###...

Tile 3433:
.##..#....
...###.###
.......##.
#..#.#.#.#
........#.
...###...#
...#......
.##...###.
.#.#..#..#
##..##..#.

Tile 1091:
###.####..
.#..#.....
##.......#
#.#...#...
...##.....
#..###...#
..##...#.#
#...#..###
.....#...#
###.#....#

Tile 3407:
.####.#...
.#..#.#..#
..#..#....
......#..#
.........#
#...##...#
...#.###.#
#....#...#
#.....###.
.#.#.#.#..

Tile 2591:
.##..##...
..#...##.#
...#..####
.#....#.##
#...##.#..
#...#....#
#..#.....#
#..#.....#
..........
.#.###.#.#

Tile 3613:
###.######
.#...##..#
#.##.....#
#.........
#...#.....
###.....##
.#.##.#..#
##..#...##
####......
..#...#...

Tile 3967:
#.#..#.#..
#..#.....#
...##...#.
.#.##...#.
#####..#.#
..##.#..#.
..........
#.#.#..#.#
#.#.....##
#..#####.#

Tile 1999:
#.#.#..##.
####.##.##
..##...#.#
....#....#
.##.#.#...
..........
.......##.
#.#......#
##...#.#.#
.#.#.#.#..

Tile 2689:
.#..#####.
.#........
..#....#.#
#......#.#
.#.#..#.##
###...#..#
...#...#.#
##......##
......##.#
.##.#..##.

Tile 3533:
.####.#...
#........#
#......#..
.........#
#..#.##..#
###....#.#
...##..#..
..#.....#.
#...#..###
..###.##.#

Tile 2267:
#.##...#.#
#....#....
.#.#.##...
.#....#.##
..#.#.....
.##.......
##.....#..
####....#.
......#..#
#..##..#.#

Tile 2297:
###..##...
#..##..##.
..#....#.#
#.#.##...#
..#.##..#.
...##...#.
#...#.....
.....#.#.#
#..#....#.
.###..##..

Tile 2711:
##..##...#
...##....#
...#..#.#.
#........#
...#.....#
........##
##....##..
#.##.....#
#.###.....
###...#..#

Tile 1931:
.###...###
......#.##
....##..##
#...#...#.
..#..##.#.
#.....#..#
#...#....#
#........#
...#..#.#.
#.##..###.

Tile 1787:
#...##..##
..#.......
#..#.#.#..
...#..#..#
.##.#..###
#.#.#.....
..........
....##...#
..........
.#.#...#..

Tile 2549:
##.#...###
.....#...#
#......###
##.###....
#..#....#.
##.......#
.#........
..###...##
##..##...#
#.##...#.#

Tile 2789:
#.#.#.#.##
#...#.##..
.##...#.##
#.##.#....
...#......
###.......
#.......##
#.#...#..#
..#.#....#
.##.#..#..

Tile 2707:
#..#.#...#
#.#.#.#...
...#..#..#
##.#.....#
#....#.##.
.....#...#
#.....#...
....#.#...
#..###...#
##..##.###

Tile 3313:
###.#..#..
.#....#..#
...#.....#
....#..###
....#..#..
#..#.#....
##.#..##..
..#...#.##
##..##....
.##.###.#.

Tile 1607:
..#...###.
#.....#...
#.......##
#.....#...
###.#.#..#
#.#.....#.
#..#...###
.........#
..#.......
##.#.#####

Tile 3889:
###.#...##
##.......#
....#..#..
#.#......#
.##......#
#...##....
#....##..#
.......#..
#...#...#.
##.#####.#

Tile 3821:
..###.##..
........##
#..##..#.#
.##.#..#.#
#....##.##
#.....##.#
#.....#..#
##........
#.#..#....
##.#.###..

Tile 3347:
.##..#...#
#.#...#...
#..#..###.
.#......#.
#..###.##.
......#...
##..##.#..
.#.###.#..
........#.
#.###..##.

Tile 1907:
.#.##...#.
#......#..
...#......
##.......#
.#.....#..
.#.....###
........##
.##.#..#.#
##.....#..
#.########

Tile 2903:
##.....#.#
....#.##..
...#.....#
.#..#####.
.....##.#.
.#......#.
..........
#..#.....#
....#...##
.#..##..##

Tile 1399:
#....###.#
..#..#####
.#....##.#
...#...#..
#..##.#..#
.....#....
..#......#
........##
#..###....
#..###.###

Tile 1993:
###.##..#.
##........
##..#..##.
.........#
.........#
#..##.#.#.
.##.......
#..#.....#
#.##.#..##
##.#.#..#.

Tile 3671:
...##.#..#
##....##.#
#..#....##
#..#...#.#
.#.#.#...#
........##
..........
##.......#
#.##....##
.#..#..##.

Tile 3167:
###..##..#
.....##...
.......#..
#...#.#..#
.#.##.#..#
###....##.
##........
#.........
..#..#....
.....##.#.

Tile 1109:
..#...#..#
#..#.#.#..
.#........
.......#.#
#..#..#...
.........#
##.#......
........##
#.#.##..#.
..#..#.#..

Tile 3089:
.###.#####
#..#..#.#.
#......#.#
...#.....#
.........#
.#.#...#..
.#..#.#.##
.......###
.#.#..#.##
..#..###..

Tile 1051:
.....##..#
#...##.#..
..........
..........
#.#.##..#.
...##...##
.##.#####.
.#...##..#
..#.#...#.
#.......##

Tile 2113:
#...###.##
.#.....##.
...#....##
...##..#..
#....#...#
..##.#....
##....#.#.
..#...###.
.....#.#.#
.##......#

Tile 2131:
#....#####
#.#.......
.###.#..##
###.##.#..
#...#.....
.#.#.....#
###....#..
..#.....##
.#...##.##
#..###....
";