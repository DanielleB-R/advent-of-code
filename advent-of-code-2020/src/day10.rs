use std::collections::HashMap;

pub fn build_gap_histogram(jolts: &mut [usize]) -> HashMap<usize, usize> {
    let mut histo = HashMap::new();

    jolts.sort();

    let mut iter = jolts.iter().copied().peekable();
    // We start with 0 on the port so we increment by our first value
    *histo
        .entry(iter.peek().copied().unwrap_or_default())
        .or_default() += 1;
    while let Some(size) = iter.next() {
        let diff = match iter.peek() {
            Some(next) => next - size,
            // The device is three more than the highest value
            None => 3,
        };

        *histo.entry(diff).or_default() += 1;
    }

    histo
}

// NOTE: I assume `jolts` is sorted
// We handle the n <= 2 case in the initialization
fn combinations_iterator(jolts: &[usize], cn3: usize, cn2: usize, cn1: usize, n: usize) -> usize {
    if n >= jolts.len() {
        return cn1;
    }

    let cn = if jolts[n] - jolts[n - 3] <= 3 {
        cn1 + cn2 + cn3
    } else if jolts[n] - jolts[n - 2] <= 3 {
        cn1 + cn2
    } else {
        cn1
    };

    combinations_iterator(jolts, cn2, cn1, cn, n + 1)
}

pub fn possible_combinations(jolts: &[usize]) -> usize {
    let c0 = 1;
    let c1 = if jolts[1] > 3 { 1 } else { 2 };
    let c2 = if jolts[2] > 3 {
        c1
    } else if jolts[2] - jolts[0] > 3 {
        c1 + c0
    } else {
        1 + c1 + c0
    };

    combinations_iterator(jolts, c0, c1, c2, 3)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_combinations() {
        let mut jolts = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        jolts.sort();

        assert_eq!(possible_combinations(&jolts), 8);

        let mut jolts2 = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        jolts2.sort();

        assert_eq!(possible_combinations(&jolts2), 19208);
    }
}
