use advent_of_code_2020::{self, day3};

fn main() {
    let forest =
        advent_of_code_2020::read_and_parse_from_file("day3-input.dat", day3::parse_tree_line)
            .unwrap();

    let trees_part1 = day3::find_trees_hit(&forest, 3, 1);

    println!("hit {} trees in part 1", trees_part1);

    let trees_product: usize = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(vx, vy)| day3::find_trees_hit(&forest, vx, vy))
        .product();

    println!("product of trees is {}", trees_product);
}
