use std::io::{stdin, BufRead};

type Machine = Vec<usize>;

fn read_program() -> Machine {
    for line in stdin().lock().lines() {
        return line
            .unwrap()
            .split(',')
            .map(|piece| piece.parse().unwrap())
            .collect();
    }
    vec![]
}

fn run_program(mut program: Machine) -> usize {
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
    program[0]
}

fn insert_noun_verb(base_program: &Machine, noun: usize, verb: usize) -> Machine {
    let mut program = base_program.clone();
    program[1] = noun;
    program[2] = verb;
    program
}

fn main() {
    let program = read_program();

    for noun in 0..=99 {
        for verb in 0..=99 {
            if run_program(insert_noun_verb(&program, noun, verb)) == 19690720 {
                println!("Noun: {}, Verb: {}", noun, verb);
                return;
            }
        }
    }
}
