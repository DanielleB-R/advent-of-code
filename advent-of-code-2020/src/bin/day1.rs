use advent_of_code_2020::*;

fn main() {
    let numbers = read_numbers_from_file("day1-input.dat").expect("problem with file");

    let pair = day1::find_2020_sum(&numbers, 2);
    println!("{} * {} = {}", pair[0], pair[1], pair[0] * pair[1]);

    let triplet = day1::find_2020_sum(&numbers, 3);
    println!(
        "{} * {} * {} = {}",
        triplet[0],
        triplet[1],
        triplet[2],
        triplet[0] * triplet[1] * triplet[2]
    );
}
