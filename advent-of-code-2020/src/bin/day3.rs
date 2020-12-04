use advent_of_code_2020::day3;
use std::fs;

fn main() {
    let forest: Vec<Vec<bool>> = fs::read_to_string("day3-input.dat")
        .unwrap()
        .lines()
        .map(day3::parse_tree_line)
        .collect();

    let trees_part1 = day3::find_trees_hit(&forest, 3, 1);

    println!("hit {} trees in part 1", trees_part1);

    let trees_product: usize = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(vx, vy)| day3::find_trees_hit(&forest, vx, vy))
        .product();

    println!("product of trees is {}", trees_product);
}
