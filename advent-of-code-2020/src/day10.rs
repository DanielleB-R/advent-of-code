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
fn is_adapter_omittable(jolts: &[usize]) -> Vec<bool> {
    let mut omittable = vec![false; jolts.len()];

    for i in 0..jolts.len() - 1 {
        let outer_diff = jolts.get(i + 1).copied().unwrap_or_default()
            - jolts.get(i - 1).copied().unwrap_or_default();
        omittable[i] = outer_diff <= 3;
    }

    omittable
}
