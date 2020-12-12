use regex::Regex;
use std::ops::{Add, Sub, Mul};
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
struct Vector2D(i64, i64);

const EAST: Vector2D = Vector2D(1, 0);
const WEST: Vector2D = Vector2D(-1, 0);
const NORTH: Vector2D = Vector2D(0, 1);
const SOUTH: Vector2D = Vector2D(0, -1);

impl Add for Vector2D {
    type Output = Vector2D;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2D(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<i64> for Vector2D {
    type Output = Vector2D;

    fn mul(self, rhs: i64) -> Self::Output {
        Vector2D(self.0 * rhs, self.1 * rhs)
    }
}

impl Vector2D {
    fn rotate_steps(self, steps: i64) -> Self {
        match steps % 4 {
            0 => self,
            1 | -3 => Vector2D(self.1, -self.0),
            2 | -2 => Vector2D(-self.0, -self.1),
            3 | -1 => Vector2D(-self.1, self.0),
            _ => unreachable!()
        }
    }
}


#[derive(Copy, Clone, Debug)]
enum Operation {
    East,
    South,
    West,
    North,
    Forward,
    Right,
    Left
}

#[derive(Debug)]
struct Move(Operation, i64);

#[derive(Clone, Debug)]
struct Ship {
    heading: Vector2D,
    position: Vector2D
}

impl Ship {
    fn new(heading: Vector2D) -> Self {
        Ship {
            heading,
            position: Vector2D(0,0)
        }
    }

    fn manhattan_distance(&self) -> i64 {
        self.position.0.abs() + self.position.1.abs()
    }

    fn apply_default(mut self, movement: &Move) -> Self {
        match movement {
            Move(Operation::Forward, x) => Ship { position: self.position + self.heading * *x, ..self },
            Move(Operation::Right, x) => Ship { heading: self.heading.rotate_steps(x / 90), ..self},
            Move(Operation::Left, x) => Ship { heading: self.heading.rotate_steps(-x / 90), ..self},
            Move(Operation::East, x) => Ship { position: self.position + EAST * *x, ..self },
            Move(Operation::West, x) => Ship { position: self.position + WEST * *x, ..self },
            Move(Operation::South, x) =>  Ship { position: self.position + SOUTH * *x, ..self},
            Move(Operation::North, x) =>  Ship { position: self.position + NORTH * *x, ..self},
        }
    }
    fn apply_twist(mut self, movement: &Move) -> Self {
        match movement {
            Move(Operation::Forward, x) => Ship { position: self.position + self.heading * *x, ..self },
            Move(Operation::Right, x) => Ship { heading: self.heading.rotate_steps(x / 90), ..self},
            Move(Operation::Left, x) => Ship { heading: self.heading.rotate_steps(-x / 90), ..self},
            Move(Operation::East, x) => Ship { heading: self.heading + EAST * *x, ..self },
            Move(Operation::West, x) => Ship { heading: self.heading + WEST * *x, ..self },
            Move(Operation::South, x) =>  Ship { heading: self.heading + SOUTH * *x, ..self},
            Move(Operation::North, x) =>  Ship { heading: self.heading + NORTH * *x, ..self},
        }
    }
}

pub fn solve(input: &str) {
    let parse_re = Regex::new("([NSWEFRL])(\\d+)").unwrap();
    let moves = parse_re.captures_iter(input).map(|cap| {
        let op = match cap.get(1).unwrap().as_str() {
            "R" => Operation::Right,
            "L" => Operation::Left,
            "F" => Operation::Forward,
            "N" => Operation::North,
            "S" => Operation::South,
            "W" => Operation::West,
            "E" => Operation::East,
            _ => unreachable!()
        };
        let arg = i64::from_str(cap.get(2).unwrap().as_str()).unwrap();
        Move(op, arg)
    }).collect::<Vec<_>>();

    let result_simple = moves.iter().fold(Ship::new(EAST), Ship::apply_default);
    println!("DEFAULT: final state: {:?}, distance is {}", &result_simple, result_simple.manhattan_distance());
    let result_twist = moves.iter().fold(Ship::new(Vector2D(10, 1)), Ship::apply_twist);
    println!("TWIST: final state: {:?}, distance is {}", &result_twist, result_twist.manhattan_distance())
}

pub const PUZZLE_EXAMPLE: &str = "F10
N3
F7
R90
F11";

pub const INPUT: &str = "W2
N4
R90
E3
N2
W4
S5
F83
E5
F53
S3
L90
E1
S2
N2
W5
E4
L180
E4
N1
F27
L90
F9
E3
N2
N3
R90
N5
F57
W5
R180
R180
W5
F44
L90
E5
F87
R180
F61
E4
F37
E2
F39
L180
F53
S1
W1
S2
E2
L90
W4
N5
E1
S1
F31
L90
W5
L180
W1
N5
R90
N5
R90
F94
S5
R90
S2
F94
S3
E1
E5
F9
L90
W5
F83
N2
N5
L90
F33
W4
L90
E5
S5
F23
W5
N1
E3
S1
N1
F59
N1
E1
S2
F56
S2
E5
R180
S4
R180
F46
L90
F78
E5
L180
S4
F22
S5
F32
L90
F68
L90
S3
F76
E3
F71
R90
F34
L90
W5
R90
F12
F65
N4
W5
F65
R270
F13
W2
S2
R90
N1
F14
L180
W4
N5
R180
N2
R90
S3
F1
W2
F8
L90
F98
N5
E3
R90
N3
F39
L180
F87
E3
R180
E4
R90
W4
L180
W2
L90
S1
W2
R180
N3
L90
W4
S4
L90
S4
F75
R90
R180
N4
E5
F9
F40
S3
R90
S2
F26
E2
L180
S4
N5
W1
S5
W3
F11
E2
N5
W3
S5
R90
N2
E4
L90
R90
F8
E4
R90
N2
L90
N3
F8
E2
F67
W5
F19
S3
L90
S3
L90
W1
F54
S1
R90
S4
E1
S3
L90
F14
W4
W3
F36
E5
R90
F10
W2
S1
W2
N5
W4
F64
W5
S4
F13
E5
N1
F87
E3
S4
E5
W3
F46
S5
R270
S4
E3
R90
F97
F92
E2
F17
R90
F5
N1
F89
N5
F55
R90
F51
S3
F97
L90
W5
R90
F7
L180
L180
W5
F88
W2
F26
R180
S4
F54
S1
R90
F66
R90
F6
L90
N5
L90
R90
F58
E3
F67
S1
R90
W4
N4
L90
F63
E3
R90
E4
N4
L180
N3
F34
E5
R90
W1
R90
N3
F73
N5
R90
F28
W1
W3
F38
N3
E1
S5
S2
F72
R90
F25
N3
E2
S3
F63
L270
N3
E5
R90
N4
E3
S1
F32
S5
W3
F98
E2
S5
L90
N5
W4
L90
F68
E2
F81
N2
E4
L90
E1
L90
E1
L180
W3
F99
R90
W1
S4
L90
S4
R90
N2
F17
E3
F78
W1
S2
L180
N5
L90
N2
E4
L90
W1
N2
F97
W3
S5
L180
S4
F77
L90
F55
W3
N4
E4
R90
E5
S3
L90
E1
R90
F54
L90
N5
E4
R90
F41
L90
N1
R90
E5
R180
W2
F74
L90
F88
N3
F25
L180
E2
S1
W4
N1
W5
R180
F31
E1
R180
F17
N1
W2
R180
F61
L270
W4
L180
F66
E4
F68
L90
W4
L180
E4
S1
F30
S3
E1
F93
L90
F33
N3
L90
F58
R90
R90
F23
N5
W2
N3
W4
L180
N1
F84
W5
E5
F36
W3
N3
W3
R180
W2
S3
E4
F62
L90
S2
W4
F28
E1
S5
F54
S5
R270
F35
N4
R90
F38
W4
S3
W2
R90
N2
L270
F21
R90
W5
R180
F7
W1
F72
E3
L180
E1
F42
L270
F1
R90
E4
F72
W3
R90
E4
S4
W4
R90
F98
R90
F100
R90
E1
F9
N1
F81
S5
L90
L90
W3
L90
F75
L90
F27
E3
L90
F49
F53
L90
F26
W1
F48
W1
L90
W1
L90
F71
S1
F34
S1
L90
S2
N3
L180
E1
F52
S5
R90
E4
F58
W2
R90
E5
N3
R180
F56
L90
F92
S1
E2
F68
F24
N3
F29
S4
L90
N5
L90
F48
S5
F80
R90
F34
S5
F23
F36
W2
F57
W5
N1
S2
R90
F94
L90
N2
F95
R180
N1
W1
F59
N5
F62
S4
L90
N4
E2
F55
L90
F21
E2
F52
W2
R90
N3
W5
S1
L90
W1
R90
R90
F21
E4
F47
E5
N5
W3
F34
F2
N1
L90
S3
R90
W1
N4
F49
W1
F15
E5
R90
S4
F39
N4
R90
N4
F69
E2
N5
R90
F21
W5
S5
E4
S3
F67
E3
S2
R90
F51
L90
N5
F73
S1
F18
R180
W2
N1
W5
L90
W2
R90
E2
L90
W3
L90
F13
L90
F45
R90
F85
E2
F44
F65
L90
F82
W2
L270
F65
N3
W3
R90
E3
F20
R90
S2
S3
R180
N4
F98
W5
S2
F63
R90
F88
W3
F1
S4
F39
R180
N3
F84
N4
F51
E1
N5
E3
F70
L90
N3
L180
F63
S2
L90
F16
F11
R180
F70
E2
L90
F46
N2
E1
S1
F19
N5
W1
F67
R90
F79
S2
W5
F96
N1
F53
E3
R90
E1
F78
L90
F61
E5
F85
L90
W4
F72
W1
S5
F49
W1
N1
E2
R90
E2
L90
S5
R90
E2
S4
E3
F8
R90
N3
L90
W1
F56
E1
W4
N5
R90
F47
R90
W1
R90
W5
F5";