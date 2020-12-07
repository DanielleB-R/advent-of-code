fn bit_for_char(c: char) -> usize {
    match c {
        'F' => 0,
        'B' => 1,
        'L' => 0,
        'R' => 1,
        _ => panic!("Incorrect char"),
    }
}

fn decode_string_code(code: &str) -> usize {
    code.chars()
        .map(bit_for_char)
        .fold(0, |acc, bit| (acc << 1) | bit)
}

pub fn decode_boarding_pass(input: &str) -> (usize, usize) {
    let row_code = &input[0..7];
    let column_code = &input[7..];

    (
        decode_string_code(row_code),
        decode_string_code(column_code),
    )
}

pub fn calc_seat_id((row, col): (usize, usize)) -> usize {
    row * 8 + col
}

pub fn find_gap_in(seats: &mut [usize]) -> usize {
    seats.sort();

    let mut iter = seats.iter().copied().peekable();
    while let Some(id) = iter.next() {
        let next_id = iter.peek().copied().unwrap_or_default();
        if next_id - id == 2 {
            return id + 1;
        }
    }
    panic!("no suitable gap found");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_code() {
        assert_eq!(decode_string_code("FBFBBFF"), 44);
    }
}
