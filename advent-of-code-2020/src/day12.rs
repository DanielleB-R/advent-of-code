#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

pub fn parse_instruction(line: &str) -> Instruction {
    let opcode = &line[0..1];
    let operand: usize = line[1..].parse().expect("not a non-negative integer");

    match opcode {
        "N" => Instruction::North(operand),
        "S" => Instruction::South(operand),
        "E" => Instruction::East(operand),
        "W" => Instruction::West(operand),
        "L" => Instruction::Left(operand / 90),
        "R" => Instruction::Right(operand / 90),
        "F" => Instruction::Forward(operand),
        _ => panic!("not understood opcode"),
    }
}

fn turn_left(mut current: (isize, isize), count: usize) -> (isize, isize) {
    for _ in 0..count {
        current = (current.1, -current.0);
    }
    current
}

fn turn_right(mut current: (isize, isize), count: usize) -> (isize, isize) {
    for _ in 0..count {
        current = (-current.1, current.0);
    }
    current
}

pub fn evaluate_instructions_heading(instructions: &[Instruction]) -> Vec<(isize, isize)> {
    let mut heading = (0, 1);

    instructions
        .iter()
        .copied()
        .map(|instruction| match instruction {
            Instruction::North(n) => (n as isize, 0),
            Instruction::South(n) => (-(n as isize), 0),
            Instruction::East(n) => (0, n as isize),
            Instruction::West(n) => (0, -(n as isize)),
            Instruction::Left(n) => {
                heading = turn_left(heading, n);
                (0, 0)
            }
            Instruction::Right(n) => {
                heading = turn_right(heading, n);
                (0, 0)
            }
            Instruction::Forward(n) => (heading.0 * n as isize, heading.1 * n as isize),
        })
        .collect()
}

pub fn evaluate_instructions_waypoint(instructions: &[Instruction]) -> Vec<(isize, isize)> {
    let mut waypoint = (1, 10);

    instructions
        .iter()
        .copied()
        .map(|instruction| match instruction {
            Instruction::North(n) => {
                waypoint.0 += n as isize;
                (0, 0)
            }
            Instruction::South(n) => {
                waypoint.0 -= n as isize;
                (0, 0)
            }
            Instruction::East(n) => {
                waypoint.1 += n as isize;
                (0, 0)
            }
            Instruction::West(n) => {
                waypoint.1 -= n as isize;
                (0, 0)
            }
            Instruction::Left(n) => {
                waypoint = turn_left(waypoint, n);
                (0, 0)
            }
            Instruction::Right(n) => {
                waypoint = turn_right(waypoint, n);
                (0, 0)
            }
            Instruction::Forward(n) => (waypoint.0 * n as isize, waypoint.1 * n as isize),
        })
        .collect()
}

pub fn sum_displacements(displacements: Vec<(isize, isize)>) -> (isize, isize) {
    displacements.into_iter().fold((0, 0), |acc, displacement| {
        (acc.0 + displacement.0, acc.1 + displacement.1)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_waypoint_navigation() {
        let instructions: Vec<_> = vec!["F10", "N3", "F7", "R90", "F11"]
            .into_iter()
            .map(|l| parse_instruction(&l))
            .collect();

        assert_eq!(
            sum_displacements(evaluate_instructions_waypoint(&instructions)),
            (-72, 214),
        );
    }
}
