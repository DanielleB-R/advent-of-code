use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = s.split_whitespace();

        let opcode = fields.next().ok_or("Opcode missing")?;
        let operand = fields
            .next()
            .ok_or("Operand missing")?
            .parse()
            .or(Err("Invalid operand"))?;

        match opcode {
            "acc" => Ok(Self::Acc(operand)),
            "jmp" => Ok(Self::Jmp(operand)),
            "nop" => Ok(Self::Nop(operand)),
            _ => Err("Invalid opcode"),
        }
    }
}

impl Instruction {
    fn flip(self) -> Self {
        match self {
            Self::Acc(n) => Self::Acc(n),
            Self::Jmp(n) => Self::Nop(n),
            Self::Nop(n) => Self::Jmp(n),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Processor {
    program: Vec<Instruction>,
    visited: HashSet<usize>,
    pub acc: isize,
    ip: usize,
}

impl From<Vec<Instruction>> for Processor {
    fn from(program: Vec<Instruction>) -> Self {
        Self {
            program,
            visited: Default::default(),
            acc: 0,
            ip: 0,
        }
    }
}

impl Processor {
    fn interpret(&mut self) {
        self.visited.insert(self.ip);
        match self.program[self.ip] {
            Instruction::Acc(n) => {
                self.acc += n;
                self.ip += 1;
            }
            Instruction::Jmp(n) => self.ip = (self.ip as isize + n) as usize,
            Instruction::Nop(_) => self.ip += 1,
        }
    }

    fn reset(&mut self) {
        self.visited.clear();
        self.acc = 0;
        self.ip = 0;
    }

    pub fn find_loop(&mut self) -> (isize, usize) {
        self.reset();

        while !self.visited.contains(&self.ip) {
            self.interpret()
        }

        (self.acc, self.ip)
    }

    fn is_terminating(&mut self) -> bool {
        self.reset();

        loop {
            if self.visited.contains(&self.ip) || self.ip > self.program.len() {
                return false;
            }
            if self.ip == self.program.len() {
                return true;
            }

            self.interpret();
        }
    }

    // Time for some serious brute force
    pub fn repair_instruction(&mut self) -> isize {
        for i in 0..self.program.len() {
            if let Instruction::Acc(_) = self.program[i] {
                continue;
            }

            self.program[i] = self.program[i].flip();
            if self.is_terminating() {
                return self.acc;
            }
            self.program[i] = self.program[i].flip();
        }
        panic!("One instruction should fix things")
    }
}
