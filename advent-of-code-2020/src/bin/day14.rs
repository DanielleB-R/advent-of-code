use advent_of_code_2020::{self, day14};

fn main() {
    let instructions = advent_of_code_2020::read_and_parse_from_file("day14-input.dat", |l| {
        l.parse::<day14::Instruction>().unwrap()
    })
    .expect("file system error");

    let sum_values = day14::evaluate(&instructions);

    println!("{}", sum_values);

    let sum_values_v2 = day14::evaluate_v2(&instructions);

    println!("{}", sum_values_v2);
}
