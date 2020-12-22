use std::collections::HashMap;

pub fn play_number_game(starts: &[usize], turns: usize) -> usize {
    let mut last_heard_at = HashMap::new();

    for (i, n) in starts[..(starts.len() - 1)].iter().enumerate() {
        last_heard_at.insert(*n, i + 1);
    }

    let mut last_n = *starts.last().expect("starts should be non-empty");
    for turn in (starts.len() + 1)..=turns {
        let new_n = match last_heard_at.get(&last_n) {
            Some(last_time) => turn - 1 - last_time,
            None => 0,
        };

        last_heard_at.insert(last_n, turn - 1);
        last_n = new_n;
    }
    last_n
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_number_game() {
        assert_eq!(play_number_game(&[0, 3, 6], 4), 0);
        assert_eq!(play_number_game(&[0, 3, 6], 5), 3);

        assert_eq!(play_number_game(&[1, 3, 2], 2020), 1);
        assert_eq!(play_number_game(&[2, 1, 3], 2020), 10);
    }
}
