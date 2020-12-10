use arraydeque::ArrayDeque;
use itertools::Itertools;

pub fn find_first_bad_sum(numbers: &[usize]) -> usize {
    let mut sum_candidates: ArrayDeque<[_; 25], arraydeque::behavior::Wrapping> = ArrayDeque::new();

    for n in numbers.iter().copied() {
        if sum_candidates.len() == 25 {
            if sum_candidates
                .iter()
                .copied()
                .combinations(2)
                .find(|group| group.iter().sum::<usize>() == n)
                .is_none()
            {
                return n;
            }
        }
        sum_candidates.push_back(n);
    }
    panic!("no bad sum found");
}

pub fn find_weakness(numbers: &[usize], target: usize) -> usize {
    for i in 0..numbers.len() {
        let mut partial_sum = numbers[i];
        for j in i + 1..numbers.len() {
            partial_sum += numbers[j];
            if partial_sum == target {
                let smallest = numbers[i..=j].iter().min().unwrap();
                let largest = numbers[i..=j].iter().max().unwrap();
                return smallest + largest;
            }
            if partial_sum > target {
                break;
            }
        }
    }
    panic!("should be a weakness");
}
