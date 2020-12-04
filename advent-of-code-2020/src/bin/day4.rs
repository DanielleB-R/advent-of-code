use advent_of_code_2020::day4;
use std::fs;

fn main() {
    let json_passports =
        &day4::make_json_strings(&fs::read_to_string("day4-input.dat").expect("file input error"));

    let good_part1_count = day4::good_passports_in::<day4::PassportInfoRaw>(json_passports);

    println!("we have {} good ones in part 1", good_part1_count);

    let good_part2_count = day4::good_passports_in::<day4::PassportInfo>(json_passports);

    println!("we have {} good ones in part 2", good_part2_count);
}
