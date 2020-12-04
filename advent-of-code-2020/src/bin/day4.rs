use advent_of_code_2020::day4;
use std::fs;

fn main() {
    let good_count = day4::good_passports_in(&day4::make_json_strings(
        &fs::read_to_string("day4-input.dat").unwrap(),
    ));

    println!("we have {} good ones", good_count)
}
