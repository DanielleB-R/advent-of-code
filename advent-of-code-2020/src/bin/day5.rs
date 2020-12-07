use advent_of_code_2020::{self, day5};

fn main() {
    let mut seat_ids: Vec<usize> =
        advent_of_code_2020::read_and_parse_from_file("day5-input.dat", day5::decode_boarding_pass)
            .unwrap()
            .into_iter()
            .map(day5::calc_seat_id)
            .collect();

    let max = seat_ids.iter().max().copied().unwrap();

    let gap = day5::find_gap_in(&mut seat_ids);

    println!("max is {}", max);
    println!("gap is {}", gap);
}
