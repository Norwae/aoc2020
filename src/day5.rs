use std::cmp::{max, min};
use std::collections::HashMap;
use nom::bytes::complete::tag;
use nom::character::complete::{i32 as intparse};
use nom::combinator::map;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{terminated, tuple};

const MAX: usize = 1000;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn linear_address(&self) -> usize {
        self.y as usize * MAX + self.x as usize
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

struct It {
    position: Point,
    bound_low: Point,
    bound_high: Point,
    dx: i32,
    dy: i32,
}

impl Iterator for It {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.x >= self.bound_low.x && self.position.x <= self.bound_high.x
            && self.position.y >= self.bound_low.y && self.position.y <= self.bound_high.y {
            let r = self.position.clone();
            self.position.x += self.dx;
            self.position.y += self.dy;
            Some(r)
        } else {
            None
        }
    }
}

impl Line {
    fn straight(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn iterator(&self) -> It {
        let dx = i32::signum(self.end.x - self.start.x);
        let dy = i32::signum(self.end.y - self.start.y);
        let bound_low = Point {
            x: min(self.start.x, self.end.x),
            y: min(self.start.y, self.end.y)
        };
        let bound_high = Point {
            x: max(self.start.x, self.end.x),
            y: max(self.start.y, self.end.y)
        };

        It {
            position: self.start.clone(),
            dx,
            dy,
            bound_high,
            bound_low
        }
    }
}

fn point(input: &str) -> IResult<&str, Point> {
    map(tuple((intparse, tag(","), intparse)), |(x, _, y)| Point { x, y })(input)
}

fn line(input: &str) -> IResult<&str, Line> {
    map(tuple((point, tag(" -> "), point, tag("\n"))), |(start, _, end, _)| Line { start, end })(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Line>> {
    many1(line)(input)
}

pub fn solve(input: &str) -> String {
    let mut counts =  [0usize; MAX * MAX];
    let (_, lines) = parse(input).unwrap();


    for line in lines {
        for elem in line.iterator() {
            counts[elem.linear_address()] += 1;
        }
    }

    let counts = counts.iter().filter(|v| **v > 1).count();
    format!("{:?} encounters", counts)
}