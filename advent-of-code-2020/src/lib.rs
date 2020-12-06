use std::{fs, io, path};

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;

pub fn read_and_parse_from_file<T>(
    path: impl AsRef<path::Path>,
    line_parser: impl Fn(&str) -> T,
) -> io::Result<Vec<T>> {
    Ok(fs::read_to_string(path)?.lines().map(line_parser).collect())
}
