use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bitmask {
    zeros: usize,
    ones: usize,
}

impl FromStr for Bitmask {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut zeros = 0;
        let mut ones = 0;
        for c in s.chars() {
            zeros <<= 1;
            ones <<= 1;
            match c {
                '0' => zeros |= 1,
                '1' => ones |= 1,
                'X' => {}
                _ => return Err("invalid char"),
            }
        }
        Ok(Self {
            zeros: !zeros,
            ones,
        })
    }
}

impl Bitmask {
    pub fn apply(&self, n: usize) -> usize {
        n & self.zeros | self.ones
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instruction {
    Mask(Bitmask),
    Mem(usize, usize),
}

lazy_static! {
    static ref MASK_REGEX: Regex = Regex::new("^mask = ([01X]{36})$").unwrap();
    static ref MEM_REGEX: Regex = Regex::new("^mem\\[(\\d+)\\] = (\\d+)$").unwrap();
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(mask_captures) = MASK_REGEX.captures(s) {
            return Ok(Self::Mask(mask_captures.get(1).unwrap().as_str().parse()?));
        }

        if let Some(mem_captures) = MEM_REGEX.captures(s) {
            return Ok(Self::Mem(
                mem_captures.get(1).unwrap().as_str().parse().unwrap(),
                mem_captures.get(2).unwrap().as_str().parse().unwrap(),
            ));
        }

        Err("invalid line")
    }
}

pub fn evaluate(instructions: &[Instruction]) -> usize {
    let mut memory = HashMap::new();
    let mut mask = Bitmask { zeros: !0, ones: 0 };

    for instruction in instructions {
        match instruction {
            Instruction::Mask(new_mask) => mask = *new_mask,
            Instruction::Mem(addr, val) => {
                memory.insert(addr, mask.apply(*val));
            }
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bitmask_use() {
        let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
            .parse::<Bitmask>()
            .unwrap();

        assert_eq!(mask.apply(11), 73);
        assert_eq!(mask.apply(101), 101);
        assert_eq!(mask.apply(0), 64);
    }
}
