use advent_of_code_2020::day6;
use std::fs;

fn main() {
    let customs_records =
        day6::parse_records(&fs::read_to_string("day6-input.dat").expect("file input error"));

    let sum_counts: usize = customs_records.iter().map(|record| record.len()).sum();

    println!("sum of counts {}", sum_counts);
}
