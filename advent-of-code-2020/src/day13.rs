use std::str::FromStr;

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
    buses: Vec<Option<usize>>,
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
            .collect();

        Ok(Self { buses })
    }
}

impl BusSequence {
    pub fn find_time(&self) -> usize {
        let (offset, max_bus) = self
            .buses
            .iter()
            .map(|&o| o.unwrap_or_default())
            .enumerate()
            .max_by_key(|&(_, bus)| bus)
            .expect("should have buses");

        for iteration in 1.. {
            let n = iteration * max_bus - offset;

            if self.evaluate_n(n) {
                return n;
            }
        }
        unreachable!();
    }

    fn evaluate_n(&self, n: usize) -> bool {
        for (i, bus) in self.buses.iter().enumerate() {
            if let Some(bus_n) = bus {
                if (n + i) % bus_n != 0 {
                    return false;
                }
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
