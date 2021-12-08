use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::IResult;
use nom::multi::{many1, many_m_n};
use nom::sequence::{preceded, terminated, tuple};


const A: u8 = 1;
const B: u8 = 2;
const C: u8 = 4;
const D: u8 = 8;
const E: u8 = 16;
const F: u8 = 32;
const G: u8 = 64;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct DigitActivation {
    bits: u8,
}

fn wire_map(definitions: [DigitActivation; 10]) -> [i32; 128] {
    let mut activations = [None; 10];

    for definition in &definitions {
        match definition.active_segments() {
            2 => activations[1] = Some(*definition),
            3 => activations[7] = Some(*definition),
            4 => activations[4] = Some(*definition),
            7 => activations[8] = Some(*definition),
            _ => ()
        }
    }

    let one = activations[1].unwrap();
    for definition in &definitions {
        if definition.active_segments() == 6 && one.bits & definition.bits != one.bits {
            activations[6] = Some(*definition)
        }

        if definition.active_segments() == 5 && one.bits & definition.bits == one.bits {
            activations[3] = Some(*definition)
        }
    }

    let six = activations[6].unwrap();
    let three = activations[3].unwrap();

    for definition in &definitions {
        // 0, 9
        if definition.active_segments() == 6 && *definition != six {
            if definition.bits & three.bits == three.bits {
                activations[9] = Some(*definition)
            } else {
                activations[0] = Some(*definition)
            }
        }

        // 2, 5
        if definition.active_segments() == 5 && *definition != three {
            if definition.bits & six.bits == definition.bits {
                activations[5] = Some(*definition)
            } else {
                activations[2] = Some(*definition)
            }
        }
    }


    let mut res = [-1; 128];
    for i in 0i32..10 {
        res[activations[i as usize].unwrap().bits as usize] = i
    }

    res
}

#[derive(Debug)]
struct Line {
    definitions: [DigitActivation; 10],
    examples: [DigitActivation; 4],
}

impl DigitActivation {
    fn active_segments(&self) -> u32 {
        self.bits.count_ones()
    }

    fn idx(self) -> usize {
        self.bits as usize
    }
}

fn digit(input: &str) -> IResult<&str, u8> {
    map(one_of("abcdefg"), |char| {
        match char {
            'a' => A,
            'b' => B,
            'c' => C,
            'd' => D,
            'e' => E,
            'f' => F,
            'g' => G,
            _ => unreachable!()
        }
    })(input)
}

fn segment_activation(input: &str) -> IResult<&str, DigitActivation> {
    map(
        many1(digit),
        |digits| DigitActivation {
            bits: digits
                .into_iter()
                .fold(0u8, |accu, next| (accu | next as u8))
        },
    )(input)
}

fn line(input: &str) -> IResult<&str, Line> {
    map(
        tuple((
            many_m_n(10, 10, terminated(segment_activation, tag(" "))),
            tag("|"),
            many_m_n(4, 4, preceded(tag(" "), segment_activation)),
            tag("\n")
        )),
        |(definitions, _, examples, _)| {
            let definitions = definitions.try_into().unwrap();
            let examples = examples.try_into().unwrap();
            Line { definitions, examples }
        },
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Line>> {
    many1(line)(input)
}

pub fn solve(input: &str) -> String {
    let (_, lines) = parse(input).unwrap();

    let part1 = lines.iter().fold(0, |count, line| {
        count + line.examples.iter().filter(|da| {
            let count = da.active_segments();
            count == 2 || count == 4 || count == 3 || count == 7
        }).count()
    });

    let part2 = lines.into_iter().fold(0i32, |accu, line| {
        let mapping = wire_map(line.definitions);
        let next = mapping[line.examples[0].idx()] * 1000
            + mapping[line.examples[1].idx()] * 100
            + mapping[line.examples[2].idx()] * 10
            + mapping[line.examples[3].idx()] * 1;

        accu + next
    });

    format!("Part 1: {}, Part 2: {}", part1, part2)
}