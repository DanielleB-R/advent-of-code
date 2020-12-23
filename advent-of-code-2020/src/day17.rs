use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::hash::Hash;
use std::ops::Add;

pub trait Coordinate: Add<Output = Self> + Sized + Copy + Eq + Hash {
    const ONE: Self;
    const MINUS_ONE: Self;

    fn max(&self, other: Self) -> Self;
    fn min(&self, other: Self) -> Self;
    fn increment(&self) -> Self {
        *self + Self::ONE
    }
    fn decrement(&self) -> Self {
        *self + Self::MINUS_ONE
    }
    // Can't use impl Trait types here :(
    fn interior(min: Self, max: Self) -> Vec<Self>;
    fn neighbourhood() -> Vec<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord3(isize, isize, isize);

impl From<(isize, isize, isize)> for Coord3 {
    fn from(input: (isize, isize, isize)) -> Self {
        Self(input.0, input.1, input.2)
    }
}

impl From<(usize, usize)> for Coord3 {
    fn from(input: (usize, usize)) -> Self {
        Self(input.0 as isize, input.1 as isize, 0)
    }
}

impl Add for Coord3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Coordinate for Coord3 {
    const ONE: Self = Coord3(1, 1, 1);
    const MINUS_ONE: Self = Coord3(-1, -1, -1);

    fn max(&self, other: Self) -> Self {
        Self(
            max(self.0, other.0),
            max(self.1, other.1),
            max(self.2, other.2),
        )
    }
    fn min(&self, other: Self) -> Self {
        Self(
            min(self.0, other.0),
            min(self.1, other.1),
            min(self.2, other.2),
        )
    }

    fn interior(min: Self, max: Self) -> Vec<Self> {
        (min.0..=max.0)
            .cartesian_product(min.1..=max.1)
            .cartesian_product(min.2..=max.2)
            .map(|((x, y), z)| (x, y, z).into())
            .collect()
    }

    fn neighbourhood() -> Vec<Self> {
        Self::interior(Self::MINUS_ONE, Self::ONE)
            .into_iter()
            .filter(|&coord| coord != Coord3(0, 0, 0))
            .collect()
    }
}

pub fn parse_initial_active<T: Coordinate + From<(usize, usize)>>(input: &str) -> HashSet<T> {
    let mut initial_state = HashSet::new();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                initial_state.insert((i, j).into());
            }
        }
    }
    initial_state
}

#[derive(Debug, Clone, Copy)]
pub struct Bounds<T: Coordinate> {
    min: T,
    max: T,
}

impl<T: Coordinate> Bounds<T> {
    fn calculate(coords: &HashSet<T>) -> Self {
        Self {
            min: coords
                .iter()
                .copied()
                .fold1(|acc, coord| coord.min(acc))
                .unwrap(),
            max: coords
                .iter()
                .copied()
                .fold1(|acc, coord| coord.max(acc))
                .unwrap()
                .clone(),
        }
    }

    fn expand(&self) -> Self {
        Self {
            min: self.min.decrement(),
            max: self.max.increment(),
        }
    }

    fn contained_points(&self) -> Vec<T> {
        T::interior(self.min, self.max)
    }
}

pub fn iterate<T: Coordinate>(previous_active: HashSet<T>) -> HashSet<T> {
    let neighbourhood = T::neighbourhood();

    let mut new_active = HashSet::new();
    for coord in Bounds::calculate(&previous_active)
        .expand()
        .contained_points()
    {
        let active_neighbour_count = neighbourhood
            .iter()
            .map(|&offset| coord + offset)
            .filter(|coord| previous_active.contains(&coord))
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
