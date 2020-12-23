use advent_of_code_2020::day17;
use std::fs;

fn main() {
    let input = fs::read_to_string("day17-input.dat").expect("file system error");

    let mut state = day17::parse_initial_active(&input);

    for _ in 0..6 {
        state = day17::iterate(state);
    }

    println!("active count: {}", state.len());
}
