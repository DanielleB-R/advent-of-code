use itertools::Itertools;
use std::collections::HashSet;

pub fn parse_union_records(input: &str) -> Vec<HashSet<char>> {
    input
        .split("\n\n")
        .map(|record| record.chars().filter(|c| c.is_ascii_alphabetic()).collect())
        .collect()
}

pub fn parse_intersection_records(input: &str) -> Vec<HashSet<char>> {
    input
        .split("\n\n")
        .map(|record| {
            record
                .lines()
                .map(|line| line.chars().collect::<HashSet<char>>())
                .fold1(|acc, set| acc.intersection(&set).copied().collect())
                .expect("there's an empty record")
        })
        .collect()
}
