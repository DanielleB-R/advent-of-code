use std::collections::HashSet;

pub fn parse_records(input: &str) -> Vec<HashSet<char>> {
    input
        .split("\n\n")
        .map(|record| record.chars().filter(|c| c.is_ascii_alphabetic()).collect())
        .collect()
}
