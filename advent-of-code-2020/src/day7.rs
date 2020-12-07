use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

lazy_static! {
    static ref SUITCASE_REGEX: Regex = Regex::new("^(.+) bags contain (.+).$").unwrap();
    static ref CONTENTS_REGEX: Regex = Regex::new("^(\\d+) (.+) bags?$").unwrap();
}

#[derive(Debug)]
pub struct SuitcaseRule {
    name: String,
    contents: Vec<(usize, String)>,
}

impl FromStr for SuitcaseRule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = SUITCASE_REGEX.captures(s).ok_or("Invalid rule format")?;

        let name = captures.get(1).unwrap().as_str().to_owned();
        let contents_spec = captures.get(2).unwrap().as_str();

        if contents_spec == "no other bags" {
            return Ok(Self {
                name,
                contents: vec![],
            });
        }

        let contents = contents_spec
            .split(", ")
            .map(|item| {
                CONTENTS_REGEX
                    .captures(item)
                    .ok_or("Invalid contents format")
            })
            .map_results(|captures| {
                (
                    captures.get(1).unwrap().as_str().parse().unwrap(),
                    captures.get(2).unwrap().as_str().to_owned(),
                )
            })
            .try_collect()?;

        Ok(Self { name, contents })
    }
}

pub fn invert_rules(rules: &[SuitcaseRule]) -> HashMap<String, Vec<String>> {
    let mut containers: HashMap<String, Vec<String>> = HashMap::new();

    for rule in rules {
        for (_, container_name) in &rule.contents {
            containers
                .entry(container_name.to_owned())
                .or_default()
                .push(rule.name.to_owned());
        }
    }

    containers
}

pub static MY_SUITCASE: &str = "shiny gold";

pub fn find_all_containers(containers: &HashMap<String, Vec<String>>) -> HashSet<String> {
    let mut found_names = HashSet::new();

    find_containers_from(containers, &mut found_names, MY_SUITCASE);

    found_names
}

fn find_containers_from(
    containers: &HashMap<String, Vec<String>>,
    found: &mut HashSet<String>,
    from: &str,
) {
    if containers.contains_key(from) {
        for name in &containers[from] {
            if !found.contains(name) {
                found.insert(name.to_owned());
                find_containers_from(containers, found, name);
            }
        }
    }
}

pub fn find_necessary_bag_count(rules: &[SuitcaseRule], name: &str) -> usize {
    rules
        .iter()
        // YUCK this probably would be better as a map
        .find(|rule| rule.name == name)
        .unwrap()
        .contents
        .iter()
        .fold(1, |acc, (n, contents_name)| {
            acc + n * find_necessary_bag_count(rules, contents_name)
        })
}
