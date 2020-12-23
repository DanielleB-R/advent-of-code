use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashSet;

pub type ActiveCoord = (isize, isize, isize);

pub fn parse_initial_active(input: &str) -> HashSet<ActiveCoord> {
    let mut initial_state = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                initial_state.insert((i as isize, j as isize, 0 as isize));
            }
        }
    }
    initial_state
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    min: ActiveCoord,
    max: ActiveCoord,
}

impl Bounds {
    fn calculate(coords: &HashSet<ActiveCoord>) -> Self {
        let max_x = coords.iter().map(|c| c.0).max().unwrap();
        let max_y = coords.iter().map(|c| c.1).max().unwrap();
        let max_z = coords.iter().map(|c| c.2).max().unwrap();

        let min_x = coords.iter().map(|c| c.0).min().unwrap();
        let min_y = coords.iter().map(|c| c.1).min().unwrap();
        let min_z = coords.iter().map(|c| c.2).min().unwrap();

        Self {
            min: (min_x, min_y, min_z),
            max: (max_x, max_y, max_z),
        }
    }

    fn expand(&self) -> Self {
        Self {
            min: (self.min.0 - 1, self.min.1 - 1, self.min.2 - 1),
            max: (self.max.0 + 1, self.max.1 + 1, self.max.2 + 1),
        }
    }

    fn contained_points(&self) -> impl Iterator<Item = ActiveCoord> {
        (self.min.0..=self.max.0)
            .cartesian_product(self.min.1..=self.max.1)
            .cartesian_product(self.min.2..=self.max.2)
            .map(|((x, y), z)| (x, y, z))
    }
}

lazy_static! {
    static ref NEIGHBOURHOOD_OFFSETS: Vec<ActiveCoord> = Bounds {
        min: (-1, -1, -1),
        max: (1, 1, 1)
    }
    .contained_points()
    .filter(|&coord| coord != (0, 0, 0))
    .collect();
}

pub fn iterate(previous_active: HashSet<ActiveCoord>) -> HashSet<ActiveCoord> {
    let mut new_active = HashSet::new();
    for coord in Bounds::calculate(&previous_active)
        .expand()
        .contained_points()
    {
        let active_neighbour_count = NEIGHBOURHOOD_OFFSETS
            .iter()
            .map(|&(ox, oy, oz)| (coord.0 + ox, coord.1 + oy, coord.2 + oz))
            .filter(|coord| previous_active.contains(coord))
            .count();

        let is_active = previous_active.contains(&coord);
        match (is_active, active_neighbour_count) {
            (true, 2..=3) => new_active.insert(coord),
            (false, 3) => new_active.insert(coord),
            _ => false,
        };
    }
    new_active
}
