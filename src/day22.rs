use std::iter::FromIterator;
use std::str::FromStr;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{eof, map};
use nom::IResult;
use nom::lib::std::collections::{HashSet, LinkedList};
use nom::multi::many1;
use nom::sequence::{terminated, tuple};
use num::BigUint;

fn end(input: &str) -> IResult<&str, &str> {
    alt((tag("\n"), eof))(input)
}

fn deck(input: &str) -> IResult<&str, Vec<u32>> {
    map(
        tuple((
            tag("Player "),
            digit1,
            tag(":\n"),
            many1(
                terminated(
                    map(digit1, |s| u32::from_str(s).unwrap()),
                    end)
            ),
            end
        )),
        |(_, _, _, v, _)| v,
    )(input)
}

fn fingerprint(deck: &LinkedList<u32>) -> BigUint {
    deck.iter().fold(BigUint::from(0u32), |accu, next| {
        accu * 64u32 + next
    })
}


fn combat_simple(player1: &mut LinkedList<u32>, player2: &mut LinkedList<u32>) {
    while !player1.is_empty() && !player2.is_empty() {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();

        if card1 > card2 {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }
    }
}


fn combat_recursive(player1: &mut LinkedList<u32>, player2: &mut LinkedList<u32>, depth: usize) -> bool {
    let mut round = 1;
    let mut fingerprints = HashSet::new();
    fingerprints.insert(fingerprint(player1));

    while !player1.is_empty() && !player2.is_empty() {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();

        let player1_winner = if card1 <= player1.len() as u32 && card2 <= player2.len() as u32 {
            let mut subgame_player1 = player1.iter().take(card1 as usize).cloned().collect();
            let mut subgame_player2 = player2.iter().take(card2 as usize).cloned().collect();

            let subgame_winner = combat_recursive(&mut subgame_player1, &mut subgame_player2, depth + 1);
            subgame_winner
        } else { card1 > card2 };

        if player1_winner {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }


        let fingerprint = fingerprint(player1);
        if !fingerprints.insert(fingerprint) {
            return true;
        }

        round += 1;
    }

    player2.is_empty()
}

pub fn solve(input: &str) {
    let (r, mut player1_init) = deck(input).unwrap();
    let (_, mut player2_init) = deck(r).unwrap();

    let mut player1 = LinkedList::from_iter(player1_init.iter().cloned());
    let mut player2 = LinkedList::from_iter(player2_init.iter().cloned());
    combat_simple(&mut player1, &mut player2);
    let winning_deck = if player1.is_empty() { &player2 } else { &player1 };
    println!("Winning score: {}", score_deck(winning_deck));

    let mut player1 = LinkedList::from_iter(player1_init.iter().cloned());
    let mut player2 = LinkedList::from_iter(player2_init.iter().cloned());
    combat_recursive(&mut player1, &mut player2, 1);
    let winning_deck = if player1.is_empty() { &player2 } else { &player1 };
    println!("Winning score: {}", score_deck(winning_deck));
}

fn score_deck<'a, I: DoubleEndedIterator<Item=&'a u32>, T: IntoIterator<IntoIter=I>>(winning_deck: T) -> u32 {
    winning_deck.into_iter().rev().enumerate().fold(0u32, |accu, (offset, value)| {
        accu + (offset + 1) as u32 * value
    })
}

pub const INPUT: &str = "Player 1:
30
42
25
7
29
1
16
50
11
40
4
41
3
12
8
20
32
38
31
2
44
28
33
18
10

Player 2:
36
13
46
15
27
45
5
19
39
24
14
9
17
22
37
47
43
21
6
35
23
48
34
26
49";