use std::io::{stdin, BufRead};

fn read_program() -> Vec<usize> {
    for line in stdin().lock().lines() {
        return line
            .unwrap()
            .split(',')
            .map(|piece| piece.parse().unwrap())
            .collect();
    }
    vec![]
}

fn main() {
    let mut program = read_program();
    let mut pc = 0;
    loop {
        match program[pc] {
            1 => {
                let output_pos = program[pc + 3];
                program[output_pos] = program[program[pc + 1]] + program[program[pc + 2]];
            }
            2 => {
                let output_pos = program[pc + 3];
                program[output_pos] = program[program[pc + 1]] * program[program[pc + 2]];
            }
            99 => break,
            _ => panic!("unknown opcode"),
        }
        pc += 4;
    }
    println!("final value: {}", program[0]);
}
