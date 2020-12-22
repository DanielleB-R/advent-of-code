use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub struct Bitmask {
    zeros: usize,
    ones: usize,
    floating_bits: Vec<usize>,
}

impl FromStr for Bitmask {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut zeros = 0;
        let mut ones = 0;
        let mut floating_bits = vec![];

        for (i, c) in s.chars().enumerate() {
            zeros <<= 1;
            ones <<= 1;
            match c {
                '0' => zeros |= 1,
                '1' => ones |= 1,
                'X' => floating_bits.push(35 - i),
                _ => return Err("invalid char"),
            }
        }
        Ok(Self {
            zeros,
            ones,
            floating_bits,
        })
    }
}

fn evaluate_floating_bits<'a>(bit_indices: &'a [usize]) -> impl Iterator<Item = usize> + 'a {
    bit_indices
        .iter()
        .map(|bit| [0, 1].iter().map(move |n| n << bit))
        .multi_cartesian_product()
        .map(|selections| selections.into_iter().fold(0, |acc, n| acc | n))
}

impl Bitmask {
    pub fn apply(&self, n: usize) -> usize {
        n & !self.zeros | self.ones
    }

    pub fn apply_with_floating(&self, n: usize) -> Vec<usize> {
        let non_floating = n & self.zeros | self.ones;
        evaluate_floating_bits(&self.floating_bits)
            .map(move |bit_choice| non_floating | bit_choice)
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
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
    let mut mask = Bitmask {
        zeros: !0,
        ones: 0,
        floating_bits: vec![],
    };

    for instruction in instructions {
        match instruction {
            Instruction::Mask(new_mask) => mask = new_mask.clone(),
            Instruction::Mem(addr, val) => {
                memory.insert(addr, mask.apply(*val));
            }
        }
    }

    memory.values().sum()
}

pub fn evaluate_v2(instructions: &[Instruction]) -> usize {
    let mut memory = HashMap::new();
    let mut mask = Bitmask {
        zeros: !0,
        ones: 0,
        floating_bits: vec![],
    };

    for instruction in instructions {
        match instruction {
            Instruction::Mask(new_mask) => mask = new_mask.clone(),
            Instruction::Mem(base, val) => {
                for addr in mask.apply_with_floating(*base) {
                    memory.insert(addr, *val);
                }
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

    #[test]
    fn test_floating_bits() {
        let mask = "000000000000000000000000000000X1001X"
            .parse::<Bitmask>()
            .unwrap();

        let mut results = mask.apply_with_floating(42);
        results.sort();
        assert_eq!(results, vec![26, 27, 58, 59]);
    }

    #[test]
    fn test_v2_decoder() {
        let instructions = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1"
            .lines()
            .map(|l| l.parse::<Instruction>().unwrap())
            .collect_vec();

        assert_eq!(evaluate_v2(&instructions), 208);
    }
}
