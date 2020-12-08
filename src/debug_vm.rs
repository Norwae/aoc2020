use regex::Regex;

#[derive(Debug, Clone)]
pub struct VirtualMachine {
    pub accu: i32,
    pub pc: usize,
    pub program: Vec<Operation>,
}

#[derive(Debug, Clone)]
pub enum Operation {
    Accu(i32),
    Jump(i32),
    Nop(i32),
}

#[derive(Debug)]
pub enum Exit {
    Abort,
    OutOfBounds,
}

#[derive(Debug, Eq, PartialEq)]
pub enum DebugOp {
    Step,
    Exit,
}

impl VirtualMachine {
    pub fn new(input: &str) -> Self {
        let command = Regex::new(r"(?m)^(acc|jmp|nop) ([+-]\d+)$").unwrap();
        let program = command.captures_iter(input).map(|l| {
            let arg = l.get(2).unwrap().as_str().parse().unwrap();
            match l.get(1).unwrap().as_str() {
                "acc" => Operation::Accu(arg),
                "jmp" => Operation::Jump(arg),
                "nop" => Operation::Nop(arg),
                _ => unreachable!()
            }
        }).collect();
        VirtualMachine { program, pc: 0, accu: 0 }
    }

    pub fn run<Debug>(&mut self, mut debug: Debug) -> Exit where Debug: FnMut(&mut Self) -> DebugOp {
        while (0usize..self.program.len()).contains(&self.pc) {
            if debug(self) == DebugOp::Exit {
                return Exit::Abort;
            }
            match self.program[self.pc] {
                Operation::Accu(d) => {
                    self.accu += d;
                    self.pc += 1;
                }
                Operation::Jump(d) => self.pc = (self.pc as i32 + d) as usize,
                Operation::Nop(_) => self.pc += 1,
            }
        }

        Exit::OutOfBounds
    }
}