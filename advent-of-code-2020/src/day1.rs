use itertools::Itertools;

pub fn find_2020_sum(numbers: &[usize], group_size: usize) -> Vec<usize> {
    numbers
        .iter()
        .cloned()
        .combinations(group_size)
        .find(|group| group.iter().sum::<usize>() == 2020)
        .expect("Group with right sum should exist")
}
