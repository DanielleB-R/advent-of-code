use advent_of_code_2020::{self, day8};

fn main() {
    let mut processor: day8::Processor =
        advent_of_code_2020::read_and_parse_from_file("day8-input.dat", |s| {
            s.parse::<day8::Instruction>().unwrap()
        })
        .expect("File system error")
        .into();

    let (acc, ip) = processor.find_loop();
    println!("loop at {} with {}", ip, acc);

    let fixed_acc = processor.repair_instruction();
    println!("fixed program yields {}", fixed_acc);
}
