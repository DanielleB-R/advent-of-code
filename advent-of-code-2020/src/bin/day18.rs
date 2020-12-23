use advent_of_code_2020::{self, day18};

fn main() {
    let expressions =
        advent_of_code_2020::read_and_parse_from_file("day18-input.dat", day18::parse_tokens)
            .expect("file system error");

    let sum_results: isize = expressions
        .iter()
        .map(|tokens| day18::evaluate_tokens(tokens))
        .sum();

    println!("sum of results: {}", sum_results)
}
