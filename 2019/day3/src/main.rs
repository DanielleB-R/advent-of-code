use std::io::{stdin, BufRead, Result};

type Point = (i32, i32);

fn intersects(pt: Point, coord: i32) -> bool {
    coord >= pt.0 && coord <= pt.1
}

fn taxicab(pt: Point) -> i32 {
    let dist = pt.0.abs() + pt.1.abs();
    if dist == 0 {
        i32::max_value()
    } else {
        dist
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Segment {
    Horizontal(Point, i32),
    Vertical(i32, Point),
}

impl Segment {
    fn intersection_with(&self, other: &Segment) -> Option<Point> {
        match (self, other) {
            (&Self::Horizontal(_, _), &Self::Horizontal(_, _)) => None,
            (&Self::Vertical(_, _), &Self::Vertical(_, _)) => None,
            (&Self::Horizontal(ph, y), &Self::Vertical(x, pv)) => {
                if intersects(ph, x) && intersects(pv, y) {
                    Some((x, y))
                } else {
                    None
                }
            }
            (&Self::Vertical(x, pv), &Self::Horizontal(ph, y)) => {
                if intersects(ph, x) && intersects(pv, y) {
                    Some((x, y))
                } else {
                    None
                }
            }
        }
    }
}

fn parse_path(line: Result<String>) -> Vec<Segment> {
    let mut x = 0;
    let mut y = 0;
    line.unwrap()
        .split(',')
        .map(|instruction| {
            let distance: i32 = instruction[1..].parse().unwrap();
            match instruction.chars().nth(0).unwrap() {
                'R' => {
                    let initial = x;
                    x += distance;
                    Segment::Horizontal((initial, x), y)
                }
                'L' => {
                    let initial = x;
                    x -= distance;
                    Segment::Horizontal((x, initial), y)
                }
                'U' => {
                    let initial = y;
                    y += distance;
                    Segment::Vertical(x, (initial, y))
                }
                'D' => {
                    let initial = y;
                    y -= distance;
                    Segment::Vertical(x, (y, initial))
                }
                _ => panic!("unknown direction"),
            }
        })
        .collect()
}

fn main() {
    let mut paths: Vec<Vec<Segment>> = stdin().lock().lines().map(parse_path).collect();
    let path2 = paths.pop().unwrap();
    let path1 = paths.pop().unwrap();

    let min_distance = path1
        .iter()
        .flat_map(|seg1| {
            path2
                .iter()
                .filter_map(move |seg2| seg1.intersection_with(seg2))
        })
        .map(taxicab)
        .min()
        .unwrap();

    println!("minimum distance is {}", min_distance);
}
