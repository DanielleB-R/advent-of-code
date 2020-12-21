use advent_of_code_2020::{self, day11};

fn main() {
    let map =
        advent_of_code_2020::read_and_parse_from_file("day11-input.dat", day11::parse_input_line)
            .expect("file system error");

    let steady_state_map = day11::find_steady_state(map);

    println!(
        "occupied seats: {}",
        steady_state_map
            .into_iter()
            .map(|row| row
                .into_iter()
                .map(|o| o.unwrap_or_default() as usize)
                .sum::<usize>())
            .sum::<usize>()
    )
}
