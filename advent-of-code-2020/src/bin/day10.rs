use advent_of_code_2020::{self, day10};

fn main() {
    let mut jolts = advent_of_code_2020::read_and_parse_from_file("day10-input.dat", |line| {
        line.parse::<usize>().unwrap()
    })
    .expect("file system error");

    let histo = day10::build_gap_histogram(&mut jolts);

    println!("part1 combination: {}", histo[&1] * histo[&3]);

    let combinations = day10::possible_combinations(&jolts);

    println!("number of combinations: {}", combinations);
}
