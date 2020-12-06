use advent_of_code_2020::{self, day2};

fn main() {
    let passwords_and_rules =
        advent_of_code_2020::read_and_parse_from_file("day2-input.dat", |line| {
            let (rulespec, password_piece) =
                line.split_at(line.find(':').expect("line did not have expected format"));
            (
                rulespec
                    .parse::<day2::PasswordRule>()
                    .expect("rule is malformed"),
                password_piece
                    .strip_prefix(": ")
                    .expect("password has wrong prefix")
                    .to_owned(),
            )
        })
        .expect("problem with file");

    let valid_part1_count = passwords_and_rules
        .iter()
        .filter(|(rule, password)| rule.matches(password))
        .count();

    println!("{} passwords valid in part 1", valid_part1_count);

    let valid_part2_count = passwords_and_rules
        .iter()
        .filter(|(rule, password)| rule.matches_new_logic(password))
        .count();

    println!("{} passwords valid in part 2", valid_part2_count);
}
