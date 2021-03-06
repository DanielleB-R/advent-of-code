use advent_of_code_2020::day17;
use std::fs;

fn main() {
    let input = fs::read_to_string("day17-input.dat").expect("file system error");

    let mut state3 = day17::parse_initial_active::<day17::Coord3>(&input);

    for _ in 0..6 {
        state3 = day17::iterate(state3);
    }

    println!("active count 3 dim: {}", state3.len());

    let mut state4 = day17::parse_initial_active::<day17::Coord4>(&input);

    for _ in 0..6 {
        state4 = day17::iterate(state4);
    }

    println!("active count 4 dim: {}", state4.len());
}
