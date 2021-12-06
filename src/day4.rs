use std::fmt::{Display, format, Formatter};
use std::mem::swap;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0, space1};
use nom::combinator::{map, opt};
use nom::IResult;
use nom::sequence::{terminated, tuple};
use nom::character::complete::{u32 as u32_parse};
use nom::multi::{many1, separated_list1};

struct Board {
    numbers: [[u32; 5]; 5],
    scored: [[bool; 5]; 5],
    has_won: bool
}


impl Board {

    fn unmarked_sum(&self) -> u32 {
        let mut sum = 0u32;
        for i in 0usize..5 {
            for j in 0usize..5 {
                if !self.scored[i][j] {
                    sum += self.numbers[i][j]
                }
            }
        }

        sum
    }

    fn apply_nr(&mut self, value: u32) -> bool {
        for i in 0usize..5 {
            for j in 0usize..5 {
                if self.numbers[i][j] == value {
                    self.scored[i][j] = true;
                }
            }
        }

        for i in 0..5 {
            if self.scored[i][0] && self.scored[i][1] && self.scored[i][2] && self.scored[i][3] && self.scored[i][4] {
                return true
            }

            if self.scored[0][i] && self.scored[1][i] && self.scored[2][i] && self.scored[3][i] && self.scored[4][i] {
                return true
            }
        }

        false
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut fmt_line = |l: usize| {
            let mut fmt_single = |c: usize| {
                if self.scored[l][c] {
                    "    X".to_string()
                } else {
                    format!("{:>5}", self.numbers[l][c])
                }
            };
            f.write_str(&format!("{} {} {} {} {}\n",
                                 fmt_single(0),
                                 fmt_single(1),
                                 fmt_single(2),
                                 fmt_single(3),
                                 fmt_single(4),
            ))
        };

        fmt_line(0)?;
        fmt_line(1)?;
        fmt_line(2)?;
        fmt_line(3)?;
        fmt_line(4)
    }
}

fn line(input: &str) -> IResult<&str, [u32; 5]> {
    terminated(map(tuple(
        (
            space0,
            u32_parse,
            space1,
            u32_parse,
            space1,
            u32_parse,
            space1,
            u32_parse,
            space1,
            u32_parse
        )
    ), |(_, _1, _, _2, _, _3, _, _4, _, _5)| {
        [_1, _2, _3, _4, _5]
    }), tag("\n"))(input)
}


fn board(input: &str) -> IResult<&str, Board> {
    map(tuple((line, line, line, line, line)), |(_1, _2, _3, _4, _5)| Board { numbers: [_1, _2, _3, _4, _5], scored: [[false;5];5], has_won: false })(input)
}


fn parse(input: &str) -> IResult<&str, (Vec<u32>, Vec<Board>)> {
    tuple(
        (
            terminated(separated_list1(tag(","), u32_parse), tag("\n\n")),
            many1(terminated(board, opt(tag("\n"))))
        )
    )(input)
}

pub fn solve(input: &str) -> String {
    let (_, (nrs, mut boards)) = parse(input).unwrap();
    let mut remaining_boards = (0..boards.len()).collect::<Vec<_>>();
    let mut next_boards = Vec::with_capacity(remaining_boards.len());
    let mut last_scoring: Option<(usize, u32)> = None;

    for nr in nrs {
        for board_idx in &remaining_boards {
            let board_idx = *board_idx;
            let board = &mut boards[board_idx];
            if board.apply_nr(nr) {
                last_scoring = Some((board_idx, nr))
                // return format!("Board {} won with marked sum {} and nr {} for score {}", board_idx, board.unmarked_sum(), nr, board.unmarked_sum() * nr);
            } else {
                next_boards.push(board_idx)
            }
        }

        if next_boards.is_empty() {
            break;
        }

        swap(&mut next_boards, &mut remaining_boards);
        next_boards.clear();
    }

    let (last_idx, last_nr) = last_scoring.unwrap();
    format!("Last board scored had score {}", boards[last_idx].unmarked_sum() * last_nr)
}