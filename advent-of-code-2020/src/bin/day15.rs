use advent_of_code_2020::day15;
use itertools::Itertools;

static INPUT: &str = "5,1,9,18,13,8,0";

fn main() {
    let starts = INPUT
        .split(',')
        .map(|l| l.parse::<usize>().unwrap())
        .collect_vec();

    let target_number = day15::play_number_game(&starts, 2020);

    println!("{}", target_number);

    let target_big_number = day15::play_number_game(&starts, 30000000);

    println!("{}", target_big_number);
}
