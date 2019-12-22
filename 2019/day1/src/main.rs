use std::io::{self, BufRead};

fn fuel_for(mass: isize) -> isize {
    mass / 3 - 2
}

fn add_fuel_for(fuel: isize) -> isize {
    let new_fuel = fuel_for(fuel);

    if new_fuel < 0 {
        0
    } else {
        new_fuel + add_fuel_for(new_fuel)
    }
}

fn main() {
    let stdin = io::stdin();
    let total: isize = stdin
        .lock()
        .lines()
        .map(|line| add_fuel_for(line.unwrap().parse().unwrap()))
        .sum();
    println!("Total fuel is {}", total);
}
