use advent_of_code_2020::{self, day7};

fn main() {
    let rules: Vec<day7::SuitcaseRule> =
        advent_of_code_2020::read_and_parse_from_file("day7-input.dat", |line| {
            line.parse().unwrap()
        })
        .expect("file error");

    let containers = day7::invert_rules(&rules);
    let containing_colors = day7::find_all_containers(&containers);

    println!("{}", containing_colors.len());

    println!(
        "{}",
        // The count includes the outermost bag
        day7::find_necessary_bag_count(&rules, day7::MY_SUITCASE) - 1
    );
}
