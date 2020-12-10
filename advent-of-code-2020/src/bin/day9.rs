use advent_of_code_2020::{self, day9};

fn main() {
    let numbers = advent_of_code_2020::read_and_parse_from_file("day9-input.dat", |line| {
        line.parse::<usize>().unwrap()
    })
    .expect("file system error");

    let bad_sum = day9::find_first_bad_sum(&numbers);

    println!("first bad sum: {}", bad_sum);

    println!(
        "encryption weakness: {}",
        day9::find_weakness(&numbers, bad_sum)
    );
}
