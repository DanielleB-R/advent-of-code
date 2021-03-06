use advent_of_code_2020::day6;
use std::fs;

fn main() {
    let input = fs::read_to_string("day6-input.dat").expect("file input error");
    let customs_records = day6::parse_union_records(&input);

    let sum_counts: usize = customs_records.iter().map(|record| record.len()).sum();

    println!("sum of counts {}", sum_counts);

    let intersection_records = day6::parse_intersection_records(&input);

    let sum_intersection_counts: usize =
        intersection_records.iter().map(|record| record.len()).sum();

    println!("sum of counts {}", sum_intersection_counts);
}
