use std::str::FromStr;

use serde::de::value::I32Deserializer;

#[derive(Debug, Clone)]
pub struct ScheduleNotes {
    earliest_time: usize,
    buses: Vec<usize>,
}

impl FromStr for ScheduleNotes {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let earliest_time = lines
            .next()
            .ok_or("should have a first line")?
            .parse()
            .map_err(|_| "first line should be a number")?;
        let buses = lines
            .next()
            .ok_or("should have a second line")?
            .split(',')
            .filter_map(|piece| piece.parse().ok())
            .collect();

        Ok(Self {
            earliest_time,
            buses,
        })
    }
}

impl ScheduleNotes {
    pub fn find_next_bus(&self) -> (usize, usize) {
        self.buses
            .iter()
            .copied()
            .map(|bus| {
                let bus_wait = (self.earliest_time / bus + 1) * bus - self.earliest_time;
                (bus, bus_wait)
            })
            .min_by_key(|&(_, wait)| wait)
            .expect("should have some buses")
    }
}

#[derive(Debug, Clone)]
pub struct BusSequence {
    buses: Vec<(usize, usize)>,
}

impl FromStr for BusSequence {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next();
        let buses = lines
            .next()
            .ok_or("should have a second line")?
            .split(',')
            .map(|piece| piece.parse().ok())
            .enumerate()
            .filter_map(|(n, o)| o.map(|bus| (n, bus)))
            .collect();

        Ok(Self { buses })
    }
}

impl BusSequence {
    pub fn find_time(&self) -> usize {
        let mut increment = self.buses[0].1 as isize;
        let mut offset = 0;
        let mut first = 0;

        for i in 2..=self.buses.len() {
            first = Self::find_subset_time(&self.buses[0..i], increment, offset);
            let second = Self::find_subset_time(&self.buses[0..i], increment, first);
            increment = second - first;
            offset = first - increment;
        }
        first as usize
    }

    fn find_subset_time(buses: &[(usize, usize)], increment: isize, offset: isize) -> isize {
        for iteration in 1.. {
            let n = (iteration as isize) * increment + offset;

            if Self::evaluate_n(buses, n as usize) {
                return n;
            }
        }
        unreachable!();
    }

    fn evaluate_n(buses: &[(usize, usize)], n: usize) -> bool {
        for (i, bus) in buses {
            if (n + i) % bus != 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_time() {
        let sequence = "\n7,13,x,x,59,x,31,19".parse::<BusSequence>().unwrap();

        assert_eq!(sequence.find_time(), 1068781);
    }
}
