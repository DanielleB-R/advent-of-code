use advent_of_code_2020::day13;
use std::fs;

fn main() {
    let input = fs::read_to_string("day13-input.dat").expect("file system error");
    let notes = input
        .parse::<day13::ScheduleNotes>()
        .expect("should parse right");

    let (bus, wait) = notes.find_next_bus();

    println!("{} {} {}", bus, wait, bus * wait);

    let sequence = input
        .parse::<day13::BusSequence>()
        .expect("should parse right");

    let n = sequence.find_time();

    println!("{}", n);
}
