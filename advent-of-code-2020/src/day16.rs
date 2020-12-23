use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref FIELD_REGEX: Regex = Regex::new("^(.+): (\\d+)-(\\d+) or (\\d+)-(\\d+)$").unwrap();
}

#[derive(Debug, Clone)]
pub struct Field {
    name: String,
    first: RangeInclusive<usize>,
    second: RangeInclusive<usize>,
}

impl FromStr for Field {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = FIELD_REGEX.captures(s).ok_or("invalid field")?;

        Ok(Self {
            name: captures.get(1).unwrap().as_str().to_owned(),
            first: (captures.get(2).unwrap().as_str().parse().unwrap())
                ..=(captures.get(3).unwrap().as_str().parse().unwrap()),
            second: (captures.get(4).unwrap().as_str().parse().unwrap())
                ..=(captures.get(5).unwrap().as_str().parse().unwrap()),
        })
    }
}

impl Field {
    pub fn contains(&self, n: &usize) -> bool {
        self.first.contains(n) || self.second.contains(n)
    }
}

pub fn parse_input(input: &str) -> (Vec<Field>, Vec<usize>, Vec<Vec<usize>>) {
    let mut parts = input.split("\n\n");
    let fields = parts.next().expect("should have fields");
    let my_ticket = parts.next().expect("should have my ticket");
    let nearby_tickets = parts.next().expect("should have nearby tickets");

    let parsed_fields = fields
        .lines()
        .map(|l| l.parse().expect("should have valid fields"))
        .collect();

    let parsed_ticket = my_ticket
        .lines()
        .nth(1)
        .expect("should have a field line")
        .split(',')
        .map(|l| l.parse().expect("should be valid"))
        .collect();

    let parsed_nearby = nearby_tickets
        .lines()
        .skip(1)
        .map(|l| {
            l.split(",")
                .map(|l| l.parse().expect("should be valid"))
                .collect()
        })
        .collect();

    (parsed_fields, parsed_ticket, parsed_nearby)
}

pub fn find_invalid_values(fields: &[Field], tickets: &[Vec<usize>]) -> Vec<usize> {
    tickets
        .iter()
        .map(|ticket| ticket.iter())
        .flatten()
        .filter(|value| fields.iter().all(|field| !field.contains(value)))
        .copied()
        .collect()
}

pub fn discard_invalid_tickets(fields: &[Field], tickets: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    tickets
        .into_iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|value| fields.iter().any(|field| field.contains(value)))
        })
        .collect()
}

pub fn classify(fields: &[Field], tickets: &[Vec<usize>]) -> HashMap<String, usize> {
    let mut field_options: HashMap<String, HashSet<usize>> = HashMap::new();

    for field in fields {
        let index = find_field_options(field, tickets);
        field_options.insert(field.name.clone(), index);
    }

    let mut field_indices = HashMap::new();

    while field_options.len() > 0 {
        let name = field_options
            .iter()
            .find(|(_, options)| options.len() == 1)
            .expect("should be a next possibility")
            .0
            .to_owned();
        let (name, index_set) = field_options.remove_entry(&name).unwrap();
        let index = *index_set.iter().next().expect("should be one");

        field_indices.insert(name, index);
        for (_, indices) in &mut field_options {
            indices.remove(&index);
        }
    }

    field_indices
}

fn find_field_options(field: &Field, tickets: &[Vec<usize>]) -> HashSet<usize> {
    (0..tickets[0].len())
        .filter(|&i| {
            tickets
                .iter()
                .map(|ticket| ticket[i])
                .all(|value| field.contains(&value))
        })
        .collect()
}
