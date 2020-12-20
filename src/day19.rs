use std::borrow::Cow;
use std::str::FromStr;

use nom::{IResult, Parser};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, one_of, space1};
use nom::combinator::{all_consuming, eof, map, map_res, rest};
use nom::lib::std::collections::{HashMap, HashSet};
use nom::multi::{many1, many_m_n, separated_list1};
use nom::sequence::{delimited, terminated, tuple};
use nom::lib::std::hash::Hash;
use std::mem;

#[derive(Debug)]
enum RuleDefinition<'a> {
    Terminal(&'a [u8]),
    Concat(Vec<u32>),
    Alterative(Box<RuleDefinition<'a>>, Box<RuleDefinition<'a>>),
}

fn merge<'a>(target: &mut HashSet<&'a[u8]>, source: HashSet<&'a[u8]>) {
    source.into_iter().for_each(|slice|{
        target.insert(slice);
    })
}

fn single_element(elem: &[u8]) -> HashSet<&[u8]> {
    let mut hs = HashSet::with_capacity(1);
    hs.insert(elem);
    hs
}

impl <'a> RuleDefinition<'a> {
    fn matches(&self, others: &HashMap<u32, Rule>, input: &[u8]) -> bool {
        let tails = self.matches0(others, input);
        println!("Tails: {:?}", tails.iter().map(|slice|String::from_utf8_lossy(slice)));
        tails.contains(&[0u8;0][..])
    }

    fn matches0<'g>(&self, others: &HashMap<u32, Rule>, input: &'g [u8]) -> HashSet<&'g [u8]>{
        match self {
            RuleDefinition::Terminal(v) => if input.len() >= v.len() && *v == &input[..v.len()]  {
                single_element(&input[v.len()..])
            } else {
                HashSet::new()
            },
            RuleDefinition::Concat(parts) => {
                let mut this_input = single_element(input);
                let mut next_input = HashSet::new();
                for index in parts {
                    let rule = &others[index];
                    for input in this_input {
                        let continues = rule.definition.matches0(others, input);
                        continues.into_iter().for_each(|slice|{
                            next_input.insert(slice);
                        });
                    }

                    this_input = next_input;
                    next_input = HashSet::new();
                }
                this_input
            }
            RuleDefinition::Alterative(a, b) => {
                let mut cont = HashSet::new();
                merge(&mut cont, a.matches0(others, input));
                merge(&mut cont, b.matches0(others, input));

                println!("Found {} potential tails: {:?}", cont.len(), cont);
                cont
            }
        }
    }
}

#[derive(Debug)]
struct Rule<'a> {
    number: u32,
    definition: RuleDefinition<'a>
}

impl <'a> Rule<'a> {
    fn new(number: u32, definition: RuleDefinition<'a>) -> Self {
        Self { number, definition }
    }
}

#[derive(Debug)]
struct Problem<'a> {
    rules: HashMap<u32, Rule<'a>>,
    inputs: Vec<&'a [u8]>
}

impl Problem<'_> {

    fn solve(&self) {
        let count = self.inputs.iter().filter(|slice|{
            let rule_0 = &self.rules[&0];
            let is_match = rule_0.definition.matches(&self.rules, slice);
            if is_match {
                println!("{}", String::from_utf8_lossy(slice));
            }
            is_match
        }).count();

        println!("Count: {}", count)
    }
}

fn problem(input: &[u8]) -> IResult<&[u8], Problem> {
    map(
        tuple((
            many1(rule),
            tag("\n"),
            many1(test_input),
            eof)),
        |(rules, _, inputs, _)| {
            let rules = rules.into_iter().map(|r|(r.number, r)).collect();
            Problem { rules, inputs}
        }
    )(input)
}


fn test_input(input: &[u8]) -> IResult<&[u8], &[u8]> {
    terminated(alpha1, tag("\n"))(input)
}

fn rule(input: &[u8]) -> IResult<&[u8], Rule> {
    map(
        tuple((rule_number, tag(b": "), rule_definition, tag(b"\n"))),
        |(number, _, definition, _)| Rule::new(number, definition),
    )(input)
}

fn rule_definition(input: &[u8]) -> IResult<&[u8], RuleDefinition> {
    alt((terminal, alternative, concat))(input)
}

fn concat(input: &[u8]) -> IResult<&[u8], RuleDefinition> {
    map(
        separated_list1(tag(" "), rule_number),
        |l| RuleDefinition::Concat(l),
    )(input)
}

fn alternative(input: &[u8]) -> IResult<&[u8], RuleDefinition> {
    map(
        tuple((concat, tag(" | "), concat)),
        |(r1, _, r2)| RuleDefinition::Alterative(Box::new(r1), Box::new(r2)),
    )(input)
}

fn rule_number(input: &[u8]) -> IResult<&[u8], u32> {
    map_res(
        digit1,
        |b| u32::from_str(&String::from_utf8_lossy(b)),
    )(input)
}

fn terminal(input: &[u8]) -> IResult<&[u8], RuleDefinition> {
    map(delimited(tag(b"\""), alpha1, tag(b"\"")),
        |c| RuleDefinition::Terminal(c),
    )(input)
}

pub fn solve(input: &str) {
    let (_, mut parsed) = problem(input.as_bytes()).unwrap();
    parsed.rules.insert(8, rule("8: 42 | 42 8\n".as_bytes()).unwrap().1);
    parsed.rules.insert(11, rule("11: 42 31 | 42 11 31\n".as_bytes()).unwrap().1);
    parsed.solve()
}


pub const EXAMPLE_INPUT: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#;

pub const INPUT: &str = r#"0: 8 11
4: 92 5 | 5 5
123: 5 3 | 92 99
22: 103 92 | 88 5
90: 92 70 | 5 117
73: 24 92
27: 92 12 | 5 109
14: 55 5 | 4 92
109: 5 21 | 92 78
63: 92 91 | 5 64
118: 92 116 | 5 49
131: 103 92 | 111 5
115: 46 5 | 93 92
93: 92 89 | 5 107
84: 92 100 | 5 57
98: 5 40 | 92 72
39: 5 4 | 92 21
102: 59 5 | 131 92
55: 92 92 | 5 92
92: "b"
20: 92 103 | 5 21
110: 121 92 | 35 5
47: 78 5 | 4 92
48: 92 92 | 92 5
96: 92 73 | 5 1
106: 110 5 | 56 92
101: 92 83 | 5 127
105: 5 53 | 92 55
91: 117 5 | 117 92
60: 5 21 | 92 4
85: 92 55 | 5 33
103: 5 44 | 92 5
45: 108 5 | 119 92
82: 5 53 | 92 68
78: 92 92 | 5 5
58: 28 92 | 133 5
121: 92 90 | 5 14
16: 117 5 | 111 92
31: 92 114 | 5 87
112: 41 5 | 132 92
65: 53 5 | 48 92
19: 64 5 | 1 92
1: 5 24 | 92 70
77: 21 92 | 78 5
61: 92 32 | 5 102
52: 92 15 | 5 101
79: 92 75 | 5 38
64: 92 70
99: 67 92 | 23 5
94: 21 5 | 111 92
30: 92 24 | 5 117
26: 5 34 | 92 61
126: 5 45 | 92 113
50: 53 92 | 48 5
83: 5 48 | 92 111
25: 92 111 | 5 4
49: 33 5 | 21 92
40: 24 5 | 24 92
124: 92 55 | 5 4
7: 5 36 | 92 106
62: 128 5 | 60 92
97: 92 78 | 5 2
69: 2 5 | 111 92
70: 44 44
21: 5 92 | 5 5
117: 92 92
116: 92 70 | 5 2
23: 44 70
72: 92 55 | 5 24
87: 130 5 | 126 92
86: 5 115 | 92 18
36: 92 71 | 5 43
108: 59 5 | 81 92
80: 5 39 | 92 6
34: 92 9 | 5 134
119: 25 5 | 85 92
120: 5 129 | 92 47
10: 5 70 | 92 48
42: 7 92 | 86 5
125: 95 92 | 80 5
17: 65 5 | 77 92
43: 120 92 | 122 5
6: 78 5 | 88 92
81: 21 5 | 68 92
89: 5 54 | 92 47
35: 104 5 | 10 92
134: 129 5 | 76 92
66: 5 24
122: 5 66 | 92 22
53: 92 5
51: 5 111 | 92 117
68: 5 5 | 44 92
3: 1 5 | 97 92
56: 74 92 | 62 5
12: 92 117 | 5 21
57: 33 5 | 70 92
113: 118 92 | 84 5
104: 5 117
5: "a"
107: 124 92 | 90 5
9: 16 92 | 50 5
2: 5 5 | 92 44
88: 44 92 | 92 5
8: 42
41: 5 70 | 92 4
54: 5 4 | 92 78
15: 92 51 | 5 105
75: 63 92 | 112 5
67: 4 92
46: 98 92 | 19 5
132: 21 5 | 24 92
29: 94 5 | 65 92
13: 92 88 | 5 117
33: 5 92
76: 92 70 | 5 53
127: 117 5 | 55 92
71: 29 5 | 96 92
129: 5 111 | 92 88
44: 5 | 92
95: 37 92 | 105 5
100: 53 92 | 2 5
18: 125 5 | 58 92
59: 5 88 | 92 24
114: 79 92 | 26 5
11: 42 31
37: 88 92 | 68 5
38: 27 5 | 17 92
32: 40 5 | 13 92
111: 92 5 | 5 92
28: 92 30 | 5 109
128: 92 2 | 5 68
74: 92 20 | 5 82
24: 5 5
133: 5 100 | 92 69
130: 92 52 | 5 123

aaaaaaababbbbaabbbaababbbbaabbaaaaabbaaabababaabaaaabaaaaababaa
"#;
const foo: &str="
baabbabbbabbaaabababaabbbaaaaababaaaabab
bbabbbaaababaaaaaaabaaab
aabbbbbababbbbbbbbbbababbbabbbbb
bbaaaabbaabbaabaaaaababa
bbbbababbabbbbbaaaabbbab
aaaaaaaaaabababbaaaaaaabbbbabbbbbbabbabbababbbbabbbbabaabbaabbaa
babababaabbabbabbabbbaaa
abaaaaababaaaabbbabababaabbbbbbaaabbbbaa
aaaaaaabbbbabaaababbbaabaabbabbabbbbbbba
bbabbbbaaaaaaabaaaaabaabaaabbaaaababaabbbbabaabbabbbbabaabaaaabbbbbbabab
abbabaabababaababaabbbaa
aaababbaaaaabaaabbbababbabbaabbbbbabbbabbaaaabaa
abbbbaaaaababababbaabbaa
bbabbaaaaabbbbbababbaababbbabbaa
bbaaabbbbaaababbbabbabbb
abbaaaaaababbaaabaababaa
aaaaabbbbabbbbbbabbbbabb
aaaabbaaaabababaabaabbaa
aaabbababbbaaababbaaaaab
abaaaabaabaaaabbaabbaabb
bbbabbbaaaaabaabbabbbabbbbbbabbbabaaaabbabababba
bbbbaabaaaabbbbbaaaaabab
aabaaabbaababbbbbbabababbabbbaababbaaaaaabbaaababbaabbbaaabaaabbaaaababa
abbabaabaabbbabababbbaba
ababaaabbabbaabababbbbab
abaaaababbbabaaabbabaaaa
aabaaababbabbbabaaabaabb
aaaabaaabbbbbaaababaabaaabaababbabaababb
bbbabababbabbababbaaaaab
abaaaaaabbbabbbbbbaabbab
aaaabbaabaababababbbabab
bbaaaaaababbaaababbbaabb
babaaaaabaaaaaaaabbbabbb
ababbaaaaabaaabbbbbaabbbbabbabaaababbbab
baaabbabbbbabaaaabaaabab
bbbabbbbaaababbbbaaaaaaaabbabbabaaabaaba
babaabbabaabbabbbbaaabbbbbbbbabbbbbbbbbbbabbbaab
aabbbbbaaaaabaaabbbbabba
bbbbaababbaaaabbaaabaaaa
abbaaabaaabaababbbbaaaaa
babaaaaaaaababbaaaabaabb
baababbbaabababaaaaaabab
bbbbbbaabbbbabbbaaabaaaa
baabbabbaaaaabbabbbaabaabaaaabaaababbbaaabaabbab
baababbbbbbbaabaaaabaaba
baaaabbabbbaaabaaaababab
abbaabbbaaaabbaaaabaaaaa
aabbababaabbabbbababbbbbbaabbabababaabab
bbbababbbababaabbabbbaba
bbbabaaabbbabbaaaabaaaaa
babbbbbbbaababbbaabbbbab
aaaaabbbababaaababbbaaab
bbbaaababbaaabbbabbaababbbbaababbaaabbababaabbbbbbababbabbbabbba
abbaabbabaaaabbabababbba
abbbbaaaababbbbbbaabbbaa
bbbbabababbaaabaaaaaabbbbababbaaaabbbaab
aaaabbaaabaabababbaababa
bbaaabbbababaaabaaaabbba
bbbbabaababaaaaaaabababbbbabbabbabaaabba
abbaaaaaaaabbabbabbaaaab
aaaabaaaaababbabbaabbbba
aabbbbbabababaaabbbababbabaababb
baaabbabbbaababbabababba
babaabbaaababaabbaabaaaaababaabaaaabaaab
abaaaaababbbbbabaabaaaaa
abbbbbabbaabbaababbbbabaabbbaaba
baababbbaabaabbbbbbbaaaabbaabaab
bbbbbbabbaaabbbabaabbabbabbaaaab
abaaaababbbaabbabbaabbaa
abababbbbabbbbbaaaabaaab
bbbaabbbbbaaaaabaabbabaabbbbaaaaaaababbbaaabababbbbbbbba
aaaaabbbbbbaabbaabaabaab
aabbaababaaabbbaabaabbab
abbabbaaabaaabaabaabbbbb
bbbaaabbaabaaababbbaabaa
bbaabbbbbbbaaaaabaabbaaa
ababaabbaabaabbbbbabaaaa
ababaaaaaaaabbbbbabbabbbabbaaaaaaabbbaaaaabbabbabbabbaaabaabbbaabaaabbabaabaaabaabbababa
abbaaabababbbababbbaaaabaaaaabab
abbbbbabbababbbbbabbbbab
baabbaabbbbbababbbbaaaab
abbbbbaababbaabbbaabaaba
aabbaaababaaabaaaaabbbab
bbbbbbaabbababaabbbabbaabbababbaaaaababa
abaabababbaabbbbabbbbbba
bbabbbaabbbbbbabaaaaabbbabbaaabaabbbbbaaabbababaaaabaaaa
aabbababaabbbabbbabaaaab
bababaaabbbbbaaaaaaaabaaaaabaabbababaabababbababaaabbbaaabbbaaaa
bbbbbbaaaabaaabaabbbbabaaaabbbbb
bababaababbbbbaabbaaabab
babaabbabbbbbbaababbabba
ababaaabaabbbbbababaaabb
bbbbabaaabbabbabbabbaaaa
ababbaabbabbaababbbaaababaabbaaa
abaaaaabbaaaaababbabbabb
bbaababbbaabbaababaababaabbaaababaabbabbbbbbbbbbaaabbbbbbbaababaabaabbbb
bbbbbbaaabaaabaaabbbabbb
aabbbababaaababbaaaaaaba
abbabaaaaabbaababbbaaabbabababba
babbababaabaabbbaaabbbbbbbbbaabbababbbab
bababbaaabaaabaabbbabbaabbababaabbbbbaaabaabbbbbbbbbbbbbaaabbaab
ababbaaabbbbbaaabababbba
aabbaaababaabaaaabaabaab
bbaabaaabbabaabaabbababbabaababababbbbaabbbaababaabbabab
aababaababaaabaaabbbbbba
ababbbaababbabababbaaabbaaaaaaaaabaabababbaabaaabaababababaaaaabababbabababbbabaaaabbbbabaaabaab
baabaabbbaaaaabbaaabbaab
aaaabaaaabaabaaabbabaaab
bbabbaaaaaababbaabbbbaaaaaaabaaabbabbbbb
ababbaabaababbbaaabbbbaa
bababbbbbbbabaabbbbabaaaabaaaaaaaabbabaa
aaabbbbbabbbabbaabbabaaaaabbbabbbababbabbbababbaabababaa
babaabbbabbaaaaabaaabbbaaaabbbaabaababaa
aabbaababbabbbabbaaaabaa
abbbbaaabababbbbbbabbaaaababaaabbbbbbbbb
baaaaaaababbaaabaabbabba
abbbaaaabbaaaaaaabbaaaab
bbabbbbbbaaababbabaabababbbaabababbabbbaabbabbaaabbababbaabbbabbbbbabbba
abbaaaaaabbabaaaabbaabbbababbaaaaaaaaabbaabbbaaaabaaabba
aabaabaaababaabbbaaabaababaabbaabaabbbbbaabbbabbbbaaabaa
aaaababbaababbabaaaaabaaabbbbaaaaabbaaaa
bbbabaaaabaaaaaaaaabaaaa
aababbbaabbbbababbbabbbbaaabaaaa
aaaaabbabbabbabaaaabbbba
aabaababababbbbbbaaaaababaabbabbbababaaaaabababb
babbbbbaaabbbabbabaabbbb
bbabbbaaabaaaaabbbaabbbbbabaaaab
bbbbaabbabbaaaaabababbbbababaaaa
abaaabaaabbaabbaabaaabbb
aaababbaababaabbaaababab
bbabbaaabaaabbabaaaaabbaaababaaa
aabbabbbbbbbababbbbbbaabbabbabaa
aaaaabaabbbbbabbaaaaabaabbbaabbbbaabaabaabbbaaabbaabbabaaabbbbbabaaaabaaabaaabbb
abbabbaabbbababbbabbabba
babbbaababbaaabaaaabaaab
bbabbaaaabbaaabaaaaabbaaaaababaaabbaabba
abaabababaababababababba
aabbbaabbaaaaabaaabbaaba
abbaaabbbbbababaabbbaabbbabbababbbbbbbaabbbbbaabbabaabaaababbbbabbbabbaaabaabbaa
aabbbaaaabbbaabaabaabbbbabbabbaa
bbbbabaababababaabababaabbabbbbbaaaaababaababbbaaabbbbabbabbbabb
babaaababaabbabbbbaabbbbbaababbaaabbbbaa
aaabbbbbaaaaabbbaabbabba
baaabbbaaabbababbababbbbabbbaaaabbabaabbabaabbba
baaabaabaabaabbbbaababba
aaababbbbabbbbaaaaaaabbbababaaaabbabaabbbaaaabababbaabaa
bbbbbaaabaaabaabbbbaaabbaabbbababbbaabbbbaaaabbb
baaaaaaabaaabbabbabbabaa
abbaabbaabbaabbbabababab
aaaaabbaaabaaabbbbaaabab
bababbaaaaaaabaaaaaababa
aabbbabbbbbbaaaaabbbaaaabaabbbbb
aabaabbbbbbbbbaabaaaaaab
bbabbababbbbabbbbbbbabaaaaaaabbbbaaaabbb
aabaababbbbbbbaaaabbaabb
aabbaabaababbbbabbababab
bbababaabbbaaabaabbbabaa
aaaababbabaaaabbabbbaabb
abbbbbababbbbbaaaaabbbaa
ababaabbaabbababbbaaabaa
bbbbababbabbbbbbbbbbabbbaaaabaabbaaabaaa
bbbabaaabaabbabbbaaabaaa
abbaaaaaabaabbabbbaaabbaabbbabababaaabab
aaabaabaaabaaabbabbbbbbabaabbaaabaabbbbabbbbbbbbabababaaaaababababababab
babbbaababbbbbaaabaaaaabaaabababaabbaabb
bbbaaabbbbbbaaaaabbbbababababbaabababaaaabbabbbaaaabbbab
babaaabaabaaaaabababbbaa
abaaaaababbbaaaaaaabbaab
bbbaaabaaabaabababaaabba
bbbababbaabbaaabbbaabbaa
babaaabaabbabaaaaabbbbbb
bbbbababbbbabbababaaabba
aabbaabaababaabbababaabbbabaabbbabbbbabbbaaabaaa
bbbaaabaaabbbabbabbababb
ababaaabbabbbbbababbbaba
bbbbabbbaaaaaaababababaa
ababaaabbbbbabaabaaabaaa
bababababbbabbabbaaaabab
baabbaababbbbaaabbbabbba
bbabbababbbbabababbbaaab
aaabbababaabaaaaababbbab
aababababbbaaabaabbbbbba
baaabbaabaabaaabaaaabbaabbaaabbababaabbbaababaabaabbabaaababaaaaaabbbbaabbaabbbabbabbaaa
bbbaaababbbababbaabbbbbb
bbabbbaabbbbbbbbabbbbbaaabbaaabbbbbbababbbaaaabbbababaabbbbaaaaaababbaba
abaaaaaabbabbabbaaaaaabb
bbbabbaaaaabbbbbbbbbbaba
aaabbabbaaaaabbbaabaabaa
bababababaababbbaabbaabb
abbbbabaabbbbabaabaabbaa
bbaabbbbbbbabbaaaaabbbab
abaabababbbaaabbaaaabbab
bbbababaabaaaababbabbbbaaaaabbbabbabaaab
aabaaabbbbabbaaaabbbabaa
ababaabbaabbbaabbbabaaba
ababbabbaabbaaabaababbbb
aabbaababaabbabaabaaabab
babbababbabbaaabaaaaabbabbabbbaababaaaab
baabbaabaabababaabababab
bbabbbabababbaabaabbbaaa
bababbaaaabaaabbaabbabaa
abbabaabbbbbababbbbaaabaabaaabaabbabaabbbabbbaba
abbbbababbaabbbbababbbaa
aaabbbbbaaabbbbbabbbbaab
aabbababababbbbabbaababbaabaabbaabaababb
babababaaabbabbbbbaabaab
abbbabbababbaabbaabbbaaa
bababbabbbaababbaababbaa
abababbbababbbbbaabaabaa
bbbabaaabaabbaababaaaabbaaaaaaababababbaaabbbbaaabbaaaab
baaabbabababaaaababaabab
bababaaabaaabbaabaaaabab
babbbbaabaaaabbabaabababaaabbabbbabbbaabbaaabbbb
baabaabbbbbbaababbababaabbbababbbabbbbaabaaababa
bbbababbbaaabbaababababaababbaba
abaaaabaabbaabbabbaaabbbabbabaababaabababaaabbbbbbaaaaab
bbabbbaabbaababbbbaaabba
baabbaabababbabbaababaaa
aaaaabbaaabbbabbaabbabbbbababbababbabbabaaabaaaa
aaaabaaababbbbbaaaabbabbbbbbabba
aabbbbbabababaaaaababbaa
aaabbabbabbabaaaabbaaabb
bbbaabbababaaabaaaaabaab
abbbbbabaabbababaabbbbbb
aabababaaaababbbabaaabab
abbabaaabbbabbbbaabbaaaa
bbbabaabbaaaaabbabbabbababbababb
abbbbbaababbbbaabbbaababbbbbaaabbbbbaaab
bbbabaabbabaabbbabaaaaaabababababaabbbbaaaaababa
bbbbaabaabbaaababbabaaaa
abbbbabaaaaaabbaababaaaabaaaababbabbaaaa
abaaaabababaabbbaaabbaab
bbbaababbbaabbbbbbaaabba
aababbabbbbaabababbbaaab
aababbababaaabaaaaaaaaabaabbbbbb
baaaaabaabbbbaaaabbbaaba
abbaaabababbbbbaabbbbbaabababaaaaaaabbaaaaabbbbabbbaaaaaababbbabaaaaaabb
bbbababbbbaaaabaabaababb
bbbaaabbbababbbbabaabbbb
baababbbbaaabbababbbbbbb
abbbaabbbabbabaaaaababbabbbbabbbaabbbbabbbabbaaa
aaabbabaabaaabaaabbbabbabaabbaaa
bbbaabbbbbabbaaaaaaabaab
baabbaabbbaaaabbbaabbbba
ababbaaababaabbbbbabaabbabbbabbbabbbbabbbbbbabbaabbaaaabbbabababbaabaaaa
abbabbaabaababbbabbabbba
bbababaaababababbbababbabaaaabaaaabbabaaaaabbabaaaababab
aaaaabbaababbaaababbbabb
babaaabababaabaaaaababaa
baababbbaabbbbbaababbaba
bbbabababbbaabbabbabaaba
bbbaabbabbaaaabbabbababb
bbababaaaaababbaaaababaa
aabbababbabaaabaaababbaa
abbbbbaabbbaababaaabaaaa
abaaaababbabbababaaabbabababaaabbbaaaabbbbaaababbbaabaab
abbbbaaabbaaabbbaaaababa
ababaaaababbababbabababb
bbbbababbaaaaababaaabbbaaabbaabaabaabaabaabbabaa
aabaaababaababbbbaabbbaa
abbbaaaaabbaababaabaaaabbbbbaabbaabaabbbabbbaaababbbbaabbbbabaabaabaabba
bbbabababaabbabbbbabaaaa
aaaaababaabaabaabbaabaabbaabbbbbbbababab
ababaaabaababaababbbaaab
aaabbaabbabbbabaabbaababbaaaaaaababaaabbbbbbbbabbbbaabaa
abbabbabbaabaababbaaabab
bbababaaabbaaababbbbbaaaabbaabbabbaababbaabbbbbbabaaabab
bbbbaaaaabaaaaaaaabababaabbababbaaaaaabbbbbbaaaabbbabbbb
baababbbbbbbaabbbabaaabb
baaaaabbaaabbaabbaabbbbbbbaabbbbbbbaabbbbbbababbabbaabaabbbbbbbabaaabaabaabbaabb
baababbbbabbaaabbabbbbaabbaaaaaabbbaaaaabbbaabaa
baaabaabaabbbaabbabbaaaa
aaababbbaaaaabbabbabaaab
babaabbaababbaabaabaaabababbabbb
abbaaaaaabbaabbbabaabbaa
bababbababaaaabbabaabbba
babaabaaabaaaaaaaabbbaaa
abbabbaababaabbabbaababa
bbaaabbbbabbababaababaabbabbaabb
ababbbbbbaabbaabaaaaabab
bbbaaabbbabaabaabaababba
abbbaaaabbababaababaaaab
bbbbbbaabbbabbaabbbaabaa
aabaaabaabababbbbaabaaab
bbbaababaaaabbbaabaabbbbbbabaabbbbaaabbabbabbabb
aabbaaababaaaaaaabababba
bbbbabbbbaabaaaaabbbbbbb
bbbbaabaaaababbabaabbaba
baaababbbbbbbaaabbbbbbbb
bbbababbbbbbbaaaababaaaababaabaaabbaaabb
abbabaaaabbaabbbaaaababbbbbaaaababaabaab
aabaababbbbabaaabaaabbaaaabaabbbaaaababa
babaaaaababbaababbbbaabbaabbaabb
aabbababbabaabaabbbaaaab
abbabbabbaaababbaaaabbab
baaabbaabbabbbabaaabaabb
aabbaabaaaaaabbaaaaaaabb
bbbabaaabbbabbaabbaaabba
bbbaaabbbbbbaabaabbaaababaaaaaaababaaaab
abbaaababbabbbabbbabbbaabbabbbaabbaabbba
abaaaabaababaaaaababbaba
bababbbbbbbbabbbaabaaabbbaabbaabbababbbababaabab
bbaaaabbaaaaabaababaabbaaaaabbbbabaabbba
baaabbaabbbaabbbbaaaabab
aabbbabbbbbaaabababbbbab
baaaaabbaababbabaabaaaaa
babbaabbbbababaaaaaaabab
baababbbbabaaaaababbabbb
abaaaaabaaabbabaaabbaabb
ababbbbababbaabaabbabbaaabbabbaaaaababbabbaaabaa
ababaabbbabaaaaabbababbb
bbbbaabbbabbaabbbabbaaaa
bbbbbaaaaabbabbbbbbbbaba
bbbbabbbaababaabbbaababa
babaaaaabbbaabbbabbabaabbbabbabbbaaabbbb
aababbbaaabbbababababbabaabaabababbababa
baaaabbabababaaaababbbaa
babbaabaaaaaabbbaaabbbababbbbbba
bbaaaabbbbbaababbbabaaaa
aaababbbaababaababbbaaba
baaaaaaababbbbaabbbaababbbbaabbaaaaabbaaabababaabaaaabaaaaababaa
bbbbababababaabbbabaabbaaaabaabbbbbaabaa
baabaabbabbbbbabbbbbbaaaabbbbabb
bababababaabaaaaaabbaaabaababbbabbbaaaaa
abbabaabbaababbbabaabbab
aaaaabbbaabaaabaabbbbaaabbbaabbbbababaaaabbbbbbaaabababbaababaaa
bbababbbbaaaabaaabbababaabaaababbabaabab
abbbbababbaaaababaabbbab
aabbaababababbabbababbba
bababbbbababbaababbbabaa
bbbaabbbaababaabbbbbaaab
babbaaabababbaabbbabbabb
ababbbbbbabbbbbabbbabaaaababaaabaaababbabbbabbabbabababb
aabaaabbabaaaabbbaaaaaab
abaaaabbabaaaabbaababbababbaabaabbaaabaa
baaaaabbabbbaaaaaaabbbaa
bababbaaaabbbabbbaabaaba
bbbbaabaababbaaaaaabbbba
bbbbaabaaabbaaabbbbabbbbababbaba
abababbbabbaaaabaaabaabb
baaaaabbbabbbaababbbaaab
bbbbabaaaaaababbbaabbbab
bbbaaabaaabaababbaababaa
abaabababaaaabbabaaaaababababbbbbbaabbabbbbaaaaa
abaaaaabbbbaaabaabbbbbba
abbbbbaaaaaaaaababbbbbaaabaaaaababbbaaaaabbbaaba
aababbaabaabbabababbbbab
baabbaaabbbaaabaaabbbbaabbbabbabbbababababbbbabbaabaaaba
bababbbbaaabbababaabbaabbabbabbb
babaaabaaaababbbaaaaaaabbaaabbbaaabbabaa
aaaababbbbbbabbbabaaaabbabbbbabb
abbbbaaaabbbbbaaaaaababbaabbbabbbaabbaababbbabbabbbbabbaabaaabba
abbbbbabbaabaaaabbaabbba
aabbbaabaabaabbbbbabbbababbabbbabaabaaba
babaaabaababbbbbbbbaabbababaababbaaababaabbbabbbbabbabba
baabaaaabababaabbaababaa
abaaabaabaabbabbabbaabbababbaaaa
bbbbababaaabbbbbbabaaabb
bbbbaabaaabaababbbaaabba
ababbaaabaaaabbaaaaaabbaaaaaabaaaabaaabbabbbabbb
aabbbaabaaaabbaaabbbbabababbabaa
aababbbabaababbbbbababba
bbbabbbbbaaaaabbbabbaaaa
bbbbbaaaabaaabaaaababbbb
aaababbabbabbbababaabbba
bbabbbababbbaaaabaaaaaab
babbbaabaaaabaaababaabbbaabbabbbbabbabbabbbbbbbb
aaabababbbbaaaaaaaabaaba
aababbabaaaabaaaabbaaaab
baaabbabbbbababbaabaabbbbbabbabaaaaaaababaaababa
bababbabbbababaabbbbbaaabbababab
baabababbabbaabbbbaabaaa
baaababbabbbaaaaaaabaabb
bbbbbaabaaaababbabbbabbb
babbababbaaaaaaabbabbababaabbaabbbbbbbbabbaabaaabbaaaaab
abbabbaababbbbbabbaabbba
bbbbbaabbaaaaabaabbabaaaaababbaa
babaabbaaaaaaaabaaabaaaa
abbbabaaabaabbbbabbaaabbababababbbababab
bbbbabbbabbbaaaaababbabbbbbbabba
aabbbbbbabaabbbbabbaaaabbaababaabbaaabaa
abbbaaaabababababaabaabbbbbaabbbbabbbaabbaababaa
abaaabaabbbababbababaaabaabbbabbabbabaab
aababaabbaaababbbaabbaaa
babaaaaababaaaaabaaababbbbbbbbaaabbbbbbabbaabbbaaabbaabb
abbbabbaaabaaabaaaabbaab
bbabbabababbaaabbababaaaaaaabbabbbabaaba
abbbbabaaaaababbabbbbbba
baaaaaaaabbaabbbaabaabaa
bababbaabbbbababbbabbaab
abbbabbabababbaaaababbbabbaababa
baaabaabbbbabbaabaababaa
bbbabbabaaababbbbaabaaab
ababbaabbbabbbabaaaaabab
bababbaabbaababbbbbababaababbbaaaaabababbbaabbbabaababbaaabaaaaaaababaaa
abaaaaabbbbaabbbbabbaabbabaaabaabbaaaabbabaabbab
abbbabbabaaabbabbabaaaaaaabbaaabbbbbabba
bbaaaaaabbabbbabbbbbbbbb
baababbbabaaaabaaaaaabbbaaaaabab
baaaaababbababaaabbbbbba
bbaababbabbabaaabaabbbab
baaabbabbbbbbaabbaabbbbb
abbaabbbbbbaaabaabbabbba
aaaabaaaabbbbbabbababbba
bbbbbbaaababaababbbaaaab
baabaaaabbbbbbaaaaaaabbbbabaabbabbbababaaaaababa
babbababaabbaabababaabab
aaaaaaababaaaabbbaaababa
aabaaabaabbbbaaababbabbb
ababbabbbababababaaabbbabbbaabaa
babaaaaabbbbabbbabbbaaaabbabbbaabbbaaaaa
bbaaaababbbababbbbbaabbaabbababaaaaabaab
bababbaaabbbbbaabaababaa
aaabbabbababaaabbababaabaababaabaabbaabb
ababbaaabbababbbbbbaabaaaabababaabbbbababaaabbbaabbbbaaaaababbba
bbbbbbaabaabbaabaaaaaabb
aaaabbabaaaabaabbbabbabbbbaabaaaabbaabbaaababbaa
bbbababbbbbaabbaabaabbbb
ababbabbbababbaaaabaabba
babbaaabbbaaaabbabbabbba
abaabaaabbbabaaaabbababa
ababbbabbaaabaaaaaaabaabbabaaababbaaabbaaababbbbabbababb
bbbabaaaabaabaaabbabbaab
baabbabbbbaaaabbaaabbbab
babaaaaaaaaaabbaaaabaabb
abbabaabababbabbabbaaabaaaabbbabbbaabaab
aaabababbabaabbababbbabbbaaababbbaababbabbbbbababbbabababababbabababaabb
bbaaaababaaabaabbbbaaaaa
aaabbbbbbbbbbbabababbaba
aaababbaaababaabaaabbaab
bbbbabbbabbbabbabbbabbba
bbbababbabbaababababbaba
bbbabbababbbaaaaabaaaabbbbbaabaa
bbbbbaabababaabbbbababab
abababbbbaabbabbabbbbabaaabaabaa
baaabbabbababbababaabbaa
aabbbabbbaaaaaaaaaababbbaaaababa
bbaaabbabaabbaaabaababbaabaabbbb
baaaaababbaaaabbbbaababbbbababbaabbaaabbbbaabbababbabbbabaabaabaaabbabba
abbbbabaabbaabbbabbbabab
bbbbaaaaaabababaababaabaababbaba
bbabbbababaaaaabbbaaaaaaabbbbbba
aaaaabbaabbbbaaaaaaaaabb
babbbbaaaaaabaaaaabbbbaa
bbbabababababbabbaaaaabaaaabaaaaaaaabaab
abaaaaaaabbaabbaabababbbaabaabba
abbabaababaabaaabbaabaab
bbaaaabaaababaababaabbba
abaaabaaabbaabbabaaaabbb
bbbaabbbbbbbabaaababbbab
ababaabbbabbbbaaabbabaaababbbaaa
abbaaaaabbaaabbbbbaaabaa
baabaabbbaaaabbabaaaabaa
abaaaababbabbbbabbbabbba
bbaababbbababababbbbaabaaabbbbbbaabbbbab
bbbbaabababaaababbbbaaab
abbabbaabaaabbbabaaabbbb
bbabbaaaababbbbaabbaaabaabaabababbbabaaaabbbabab
abababbbaababbbabbbaabba
baaabaabababaaaabbaaabaa
abbaaababaabbaaabababaaaaabaaaabbbabaabababbababbabbaaaabaaababbbabaabab
bbbbabbbaabbaaabbbaabbab
aabbabaaaabbbabbabbabbbbaabbaaaabbaabbaabaaaabbabaaabbbababbbabaaaabbbabaaaaabbabbbbaaaa
baaaaaaababbbbaababababb
bbbabbbbaaaabaaababbbbbb
abbbbbaaaabbababbbbabaaaabababbbbbbaabbaaaaababaaaaaaabb
aaabbabababababaababbbbbabaababb
baaabbaababbabbbbbaaabab
baababbbbbabaabbbaabbbbbbaaabbbbbbaaababaaaabaabbbbbbabbbababbabaabaabbbaaaabbba
aaaababbbbaaabbbaaaabbab
aabbbabbaaaaaaabaabbbbaa
babbaabbabbbbabaaabaaaab
bababaabbbbaaabbaabbbbba
aababbbaabbabaaabbabbbbb
aabaaabbaaaababbbabbbabb
bbbaaabaabbabaaabbbbbabb
aabbaabaababaabaaabaaabaabbbbaaabaabbbab
ababaababaababbbaabbbaaa
abbaaaaaabbabbabbaabaaba
abbbbbaabaaaaabbaaaaaabb
";
