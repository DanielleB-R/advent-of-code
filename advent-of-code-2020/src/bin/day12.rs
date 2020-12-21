use advent_of_code_2020::{self, day12};

fn main() {
    let directions =
        advent_of_code_2020::read_and_parse_from_file("day12-input.dat", day12::parse_instruction)
            .expect("file system error");

    let displacements = day12::evaluate_instructions_heading(&directions);
    let total_displacements = day12::sum_displacements(displacements);

    println!(
        "taxicab distance: {}",
        total_displacements.0.abs() + total_displacements.1.abs()
    );

    let waypoint_displacements =
        day12::sum_displacements(day12::evaluate_instructions_waypoint(&directions));

    println!(
        "second taxicab distance: {}",
        waypoint_displacements.0.abs() + waypoint_displacements.1.abs()
    )
}
