#![feature(map_first_last)]
#![feature(assoc_int_consts)]
#![feature(vec_remove_item)]

/*
mod day1 {
    pub fn solve_basic(input: &[&str]) {
        let numbers: Vec<i32> = input.iter().map(|s| s.parse::<i32>().unwrap()).collect();

        for i in 0..numbers.len() - 1 {
            let me = numbers[i];
            for partner in &numbers[i + 1..] {
                if (me + partner) == 2020 {
                    println!("{} * {} = {}", me, partner, me * partner)
                }
            }
        }
    }

    pub fn solve_twist(input: &[&str]) {
        let numbers: Vec<i32> = input.iter().map(|s| s.parse::<i32>().unwrap()).collect();

        for i in 0..numbers.len() - 2 {
            for j in i..numbers.len() - 1 {
                for k in j..numbers.len() {
                    let f1 = numbers[i];
                    let f2 = numbers[j];
                    let f3 = numbers[k];
                    if f1 + f2 + f3 == 2020 {
                        println!("{} * {} * {} = {}", f1, f2, f3, f1 * f2 * f3)
                    }
                }
            }
        }
    }
}

mod day2 {
    use std::ops::{Range, RangeInclusive};
    use std::str::FromStr;

    use lazy_static::lazy_static;
    use regex::Regex;

    #[derive(Debug)]
    pub struct PasswordEntry {
        character: char,
        limit: RangeInclusive<usize>,
        password: String,
    }

    impl PasswordEntry {
        fn valid_twist(&self) -> bool {
            let first = *self.limit.start() - 1;
            let last = *self.limit.end() - 1;
            let chars = self.password.chars().collect::<Vec<char>>();
            (chars[first] == self.character) ^ (chars[last] == self.character)
        }

        fn valid_standard(&self) -> bool {
            let count = self.password.chars().filter(|x| *x == self.character).count();
            self.limit.contains(&count)
        }

        fn from(line: &str) -> PasswordEntry {
            lazy_static! {
                static ref PARSE_REGEX: Regex = Regex::new(r"(\d+)-(\d+) (\D): (\D+)").unwrap();
            }
            let extract = PARSE_REGEX.captures(line).unwrap();
            let start = extract[1].parse::<usize>().unwrap();
            let end = extract[2].parse::<usize>().unwrap();
            let character = extract[3].chars().nth(0).unwrap();
            let password = extract[4].to_owned();

            PasswordEntry {
                limit: start..=end,
                password,
                character,
            }
        }
    }

    pub fn solve_default(input: &str) {
        let valid_count = input.split('\n').map(|s| PasswordEntry::from(s)).filter(PasswordEntry::valid_standard).count();

        println!("{}", valid_count)
    }

    pub fn solve_twist(input: &str) {
        let valid_count = input.split('\n').map(|s| PasswordEntry::from(s)).filter(PasswordEntry::valid_twist).count();

        println!("{}", valid_count)
    }
}

mod day3 {
    use std::ops::Index;

    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    enum MapObject {
        Open,
        Tree,
    }

    struct MapRow {
        contents: Vec<MapObject>
    }

    impl MapRow {
        fn parse(line: &str) -> Self {
            let contents = line.chars().map(|c| match c {
                '#' => MapObject::Tree,
                _ => MapObject::Open
            }).collect();

            MapRow { contents }
        }
    }

    impl Index<usize> for MapRow {
        type Output = MapObject;

        fn index(&self, index: usize) -> &Self::Output {
            &self.contents[index % self.contents.len()]
        }
    }

    struct Map {
        rows: Vec<MapRow>
    }

    impl Map {
        fn parse(input: &str) -> Self {
            let rows = input.split('\n').map(MapRow::parse).collect();
            Map { rows }
        }
    }

    struct SlopeRun<'a> {
        map: &'a Map,
        slope: (usize, usize),
        position: (usize, usize),
    }

    impl<'a> SlopeRun<'a> {
        fn new(map: &'a Map, dx: usize, dy: usize) -> Self {
            let slope = (dx, dy);
            let position = (0, 0);

            SlopeRun { map, slope, position }
        }
    }

    impl Iterator for SlopeRun<'_> {
        type Item = MapObject;

        fn next(&mut self) -> Option<Self::Item> {
            let (x, y) = self.position;
            if y >= self.map.rows.len() {
                None
            } else {
                let res = self.map.rows[y][x];
                let (dx, dy) = self.slope;
                self.position = (x + dx, y + dy);
                Some(res)
            }
        }
    }

    pub fn solve_default(input: &str) {
        let map = Map::parse(input);
        let run = SlopeRun::new(&map, 3, 1);
        let mut trees = 0;

        for square in run {
            if square == MapObject::Tree {
                trees += 1
            }
        }

        println!("{} trees", trees)
    }

    pub fn solve_twist(input: &str) {
        let map = Map::parse(input);
        let runs = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let mut product = 1;

        for (dx, dy) in runs.iter() {
            let mut trees = 0;
            let run = SlopeRun::new(&map, *dx, *dy);
            for square in run {
                if square == MapObject::Tree {
                    trees += 1
                }
            }
            product *= trees;
        }

        println!("{} product of the runs", product)
    }
}

mod day4 {
    use std::collections::HashMap;

    use lazy_static::lazy_static;
    use regex::Regex;

    struct Passport<'a> {
        fields: HashMap<&'a str, &'a str>
    }

    lazy_static! {
        static ref VALIDATORS: HashMap<&'static str, Regex> = crate::day4::build_validator_map();
    }

    impl<'a> Passport<'a> {
        fn new(input: &'a str) -> Self {
            lazy_static! {
                static ref KEY_VALUE: Regex = Regex::new(r"([^:]+):(\S+)").unwrap();
            }

            let parts = input.split_whitespace();
            let fields = parts.map(|p| {
                let capture = KEY_VALUE.captures(p).unwrap();
                (capture.get(1).unwrap().as_str(), capture.get(2).unwrap().as_str())
            }).collect();
            Passport { fields }
        }

        fn is_valid_basic(&self) -> bool {
            VALIDATORS.keys().all(|k| self.fields.contains_key(*k))
        }

        fn is_valid_extended(&self) -> bool {
            let all_present = VALIDATORS.keys().all(|k| self.fields.contains_key(*k));
            let all_valid = self.fields.iter().all(|p| {
                let (k, v) = p;
                if let Some(validator) = VALIDATORS.get(*k) {
                    validator.is_match(v)
                } else {
                    true
                }
            });
            all_present && all_valid
        }
    }

    pub fn solve_default(input: &str) {
        let valid_passports = input.split("\n\n").map(|block| Passport::new(block)).filter(Passport::is_valid_basic).count();
        println!("{} valid passports", valid_passports)
    }

    pub fn solve_twist(input: &str) {
        let valid_passports = input.split("\n\n").map(|block| Passport::new(block)).filter(Passport::is_valid_extended).count();
        println!("{} fully valid passports", valid_passports)
    }


    fn build_validator_map() -> HashMap<&'static str, Regex> {
        fn re(input: &str) -> Regex {
            Regex::new(input).unwrap()
        }
        let mut m = HashMap::new();
        m.insert("hcl", re(r"^(#[0-9a-f]{6})$"));
        m.insert("ecl", re("^(amb|blu|brn|gry|grn|hzl|oth)$"));
        m.insert("pid", re(r"^\d{9}$"));
        m.insert("hgt", re(r"^(59|6\d|7[0-6])in|(1([5-8]\d|9[0-3]))cm$"));
        m.insert("byr", re(r"^19[2-9]\d|200[0-2]$"));
        m.insert("iyr", re(r"^201\d|2020$"));
        m.insert("eyr", re(r"^202\d|2030$"));
        m
    }
}

mod day5;
mod day6;
mod day7;

*/
mod debug_vm;
// mod day8;
// mod day9;
// mod day10;
// mod day11;
// mod day12;
// mod day13;
// mod day14;
// mod day15;
// mod day16;
// mod day17;
mod day18;

fn main() {
    day18::solve(day18::INPUT)
}
