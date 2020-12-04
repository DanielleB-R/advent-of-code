use advent_of_code_2020::{self, day2};

fn main() {
    let valid_password_count =
        advent_of_code_2020::read_and_parse_from_file("day2-input.dat", |line| {
            let (rulespec, password_piece) = line.split_at(line.find(':').unwrap());
            (
                rulespec.parse::<day2::PasswordRule>().unwrap(),
                password_piece.strip_prefix(": ").unwrap().to_owned(),
            )
        })
        .unwrap()
        .into_iter()
        .filter(|(rule, password)| rule.matches_new_logic(password))
        .count();

    println!("{} passwords valid", valid_password_count);
}
