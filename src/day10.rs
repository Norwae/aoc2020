use std::intrinsics::transmute;
use std::str::FromStr;

pub fn solve(input: &str) {
    let mut input = input.split_ascii_whitespace().map(|s| u64::from_str(s).unwrap()).collect::<Vec<_>>();
    input.push(0);
    input.sort();
    let last_adapter = *input.last().unwrap();
    input.push(last_adapter + 3);

    let mut differences = [0, 0, 0];
    for i in 0..input.len() - 1 {
        let diff = input[i + 1] - input[i];
        if !(1..4).contains(&diff) {
            panic!("Difference out of range: {}", diff)
        }
        differences[(diff - 1) as usize] += 1;
    }
    println!("Part 1, 1 step * 3-step: {}", differences[0] * differences[2]);

    let mut memo = vec![None; (last_adapter + 4) as usize];
    println!("# Paths: {}", track_arrangements(&input.as_slice(), &mut memo[..]));
}

fn can_connect(arr: &[u64], offset: usize) -> bool {
    if offset >= arr.len() {
        false
    } else {
        let j1 = arr[0];
        let j2 = arr[offset];
        j2 - j1 <= 3
    }
}

fn track_arrangements(input: &[u64], memo: &mut [Option<u64>]) -> u64 {
    if input.is_empty() {
        return 1
    }
    if let Some(v) = memo[input[0] as usize] {
        return v
    }
    let path1 = track_arrangements(&input[1..], memo);
    let result = if can_connect(input, 2) {
        let path2 = track_arrangements(&input[2..], memo);
        if can_connect(input, 3) {
            let path3 = track_arrangements(&input[3..], memo);
            path1 + path2 + path3
        } else {
            path1 + path2
        }
    } else {
        path1
    };
    memo[input[0] as usize] = Some(result);
    result
}

pub const INPUT: &str = "149
87
67
45
76
29
107
88
4
11
118
160
20
115
130
91
144
152
33
94
53
148
138
47
104
121
112
116
99
105
34
14
44
137
52
2
65
141
140
86
84
81
124
62
15
68
147
27
106
28
69
163
97
111
162
17
159
122
156
127
46
35
128
123
48
38
129
161
3
24
60
58
155
22
55
75
16
8
78
134
30
61
72
54
41
1
59
101
10
85
139
9
98
21
108
117
131
66
23
77
7
100
51";