pub fn parse_input_line(line: &str) -> Vec<Option<bool>> {
    line.chars()
        .map(|c| match c {
            '#' => Some(true),
            'L' => Some(false),
            '.' => None,
            _ => panic!("invalid input"),
        })
        .collect()
}

fn occupied(spot: Option<bool>) -> bool {
    spot.unwrap_or_default()
}

static NEIGHBOUR_OFFSETS: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn neighbours(i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
    NEIGHBOUR_OFFSETS
        .iter()
        .copied()
        .map(move |(x, y)| (x + i as isize, y + j as isize))
        .filter(|&(x, y)| x >= 0 && y >= 0)
        .map(|(x, y)| (x as usize, y as usize))
}

fn visible_seats<'a>(
    map: &'a [Vec<Option<bool>>],
    i: usize,
    j: usize,
) -> impl Iterator<Item = (usize, usize)> + 'a {
    NEIGHBOUR_OFFSETS.iter().copied().filter_map(move |(x, y)| {
        let mut xcoord = x + i as isize;
        let mut ycoord = y + j as isize;
        while (0..map.len()).contains(&(xcoord as usize))
            && (0..map[0].len()).contains(&(ycoord as usize))
        {
            if map[xcoord as usize][ycoord as usize].is_some() {
                return Some((xcoord as usize, ycoord as usize));
            }
            xcoord += x;
            ycoord += y;
        }
        None
    })
}

fn crowding(map: &[Vec<Option<bool>>], i: usize, j: usize) -> usize {
    visible_seats(map, i, j)
        .map(|(x, y)| {
            occupied(
                map.get(x)
                    .map(|row| row.get(y))
                    .flatten()
                    .copied()
                    .flatten(),
            ) as usize
        })
        .sum()
}

fn iterate_seats(map: &[Vec<Option<bool>>]) -> Vec<Vec<Option<bool>>> {
    map.iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, cell)| {
                    cell.map(|occupied| match crowding(map, i, j) {
                        0 => true,
                        5..=8 => false,
                        _ => occupied,
                    })
                })
                .collect()
        })
        .collect()
}

pub fn find_steady_state(map: Vec<Vec<Option<bool>>>) -> Vec<Vec<Option<bool>>> {
    let mut old_map = map;
    loop {
        let new_map = iterate_seats(&old_map);
        if old_map == new_map {
            return new_map;
        }
        old_map = new_map;
    }
}
