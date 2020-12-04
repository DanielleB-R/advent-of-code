use std::{fs, io, path};

pub mod day1;
pub mod day2;

pub fn read_numbers_from_file(path: impl AsRef<path::Path>) -> io::Result<Vec<usize>> {
    Ok(fs::read_to_string(path)?
        .lines()
        // TODO: yuck
        .map(|line| line.parse().unwrap())
        .collect())
}
