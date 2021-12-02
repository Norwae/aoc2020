use std::fmt::format;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::IResult;
use nom::multi::many1;
use nom::sequence::{delimited, terminated, tuple};

enum Direction {
    Forward,
    Down,
    Up
}

struct Command {
    direction: Direction,
    amount: i32
}

fn direction(i: &str) -> IResult<&str, Direction> {
    alt((
        map(tag("forward"),|_|Direction::Forward),
        map(tag("down"),|_|Direction::Down),
        map(tag("up"),|_|Direction::Up),
    ))(i)
}

fn command(i: &str) -> IResult<&str, Command> {
    map(tuple((direction, tag(" "), digit1)), |(direction, _, nr)|Command{
      direction,
        amount: nr.parse::<i32>().unwrap()
    })(i)
}

fn commands(i: &str) -> IResult<&str, Vec<Command>> {
    many1(terminated(command, tag("\n")))(i)
}

pub fn solve(input: &str) -> String {
    let (_, commands) = commands(input).unwrap();
    let mut horizontal = 0;
    let mut depth_naive = 0;
    let mut aim = 0;
    let mut depth_retained = 0;

    for Command{direction, amount} in commands {
        match direction {
            Direction::Forward => {
                horizontal += amount;
                depth_retained += aim * amount
            }
            Direction::Down => {
                depth_naive += amount;
                aim += amount
            },
            Direction::Up => {
                depth_naive -= amount;
                aim -= amount
            },
        }
    }

    format!("Solution 1: {}, Solution 2: {}", horizontal * depth_naive, horizontal * depth_retained)
}