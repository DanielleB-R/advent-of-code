use advent_of_code_2020::day2;
use std::fs;

fn main() {
    let valid_password_count = fs::read_to_string("day2-input.dat")
        .unwrap()
        .lines()
        .map(|line| {
            let (rulespec, password_piece) = line.split_at(line.find(':').unwrap());
            (
                rulespec.parse::<day2::PasswordRule>().unwrap(),
                password_piece.strip_prefix(": ").unwrap(),
            )
        })
        .filter(|(rule, password)| rule.matches_new_logic(password))
        .count();

    println!("{} passwords valid", valid_password_count);
}
