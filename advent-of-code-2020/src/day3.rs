pub fn parse_tree_line(line: &str) -> Vec<bool> {
    line.chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("invalid char"),
        })
        .collect()
}

pub fn find_trees_hit(forest: &[Vec<bool>], vx: usize, vy: usize) -> usize {
    forest
        .iter()
        .step_by(vy)
        .enumerate()
        .skip(1)
        .filter(|(step, trees)| trees[step * vx % trees.len()])
        .count()
}
