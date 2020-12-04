use custom_error::custom_error;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use std::convert::{TryFrom, TryInto};

pub fn make_json_strings(weirdo_input: &str) -> Vec<String> {
    // First we split into passport records
    weirdo_input
        .split("\n\n")
        // Then into key-value pairs
        .map(|record| {
            record
                .split_whitespace()
                .map(|pair| {
                    // keys are three chars!
                    let key = &pair[0..3];
                    let value = &pair[4..];

                    format!("\"{}\": \"{}\"", key, value)
                })
                .join(", ")
        })
        .map(|contents| "{".to_owned() + &contents + "}")
        .collect()
}

custom_error! {pub PassportParseError
               OutOfRange = "value out of range",
               BadUnit = "unit not known",
               ParseInt{source: std::num::ParseIntError} = "invalid integer",
               BadStringFormat = "incorrect string format"
}

fn check_range(value: usize, min: usize, max: usize) -> Result<(), PassportParseError> {
    if (min..max + 1).contains(&value) {
        Ok(())
    } else {
        Err(PassportParseError::OutOfRange)
    }
}

#[derive(Debug, Deserialize)]
struct PassportInfoRaw {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
struct BirthYear(usize);

impl TryFrom<String> for BirthYear {
    type Error = PassportParseError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        if input.len() != 4 {
            Err(PassportParseError::BadStringFormat)
        } else {
            let value = input.parse()?;
            check_range(value, 1920, 2002)?;
            Ok(Self(value))
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
struct IssueYear(usize);

impl TryFrom<String> for IssueYear {
    type Error = PassportParseError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        if input.len() != 4 {
            Err(PassportParseError::BadStringFormat)
        } else {
            let value = input.parse()?;
            check_range(value, 2010, 2020)?;
            Ok(Self(value))
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
struct ExpiryYear(usize);

impl TryFrom<String> for ExpiryYear {
    type Error = PassportParseError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        if input.len() != 4 {
            Err(PassportParseError::BadStringFormat)
        } else {
            let value = input.parse()?;
            check_range(value, 2020, 2030)?;
            Ok(Self(value))
        }
    }
}

#[derive(Debug)]
enum Unit {
    In,
    Cm,
}

impl TryFrom<&str> for Unit {
    type Error = PassportParseError;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        match input {
            "in" => Ok(Self::In),
            "cm" => Ok(Self::Cm),
            _ => Err(PassportParseError::BadUnit),
        }
    }
}

lazy_static! {
    static ref HEIGHT_REGEX: Regex = Regex::new("^(\\d+)(..)$").unwrap();
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
struct Height {
    value: usize,
    unit: Unit,
}

impl TryFrom<String> for Height {
    type Error = PassportParseError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        let captures = HEIGHT_REGEX
            .captures(&input)
            .ok_or(PassportParseError::BadStringFormat)?;

        let value = captures.get(1).unwrap().as_str().parse()?;
        let unit = captures.get(2).unwrap().as_str().try_into()?;

        match unit {
            Unit::In => check_range(value, 56, 76)?,
            Unit::Cm => check_range(value, 150, 193)?,
        }

        Ok(Self { value, unit })
    }
}

lazy_static! {
    static ref HCL_REGEX: Regex = Regex::new("^#[0-9a-f]{6}$").unwrap();
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
struct HairColor(String);

impl TryFrom<String> for HairColor {
    type Error = PassportParseError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        if HCL_REGEX.is_match(&input) {
            Ok(Self(input))
        } else {
            Err(PassportParseError::BadStringFormat)
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum EyeColor {
    Amb,
    Blu,
    Brn,
    Gry,
    Grn,
    Hzl,
    Oth,
}

lazy_static! {
    static ref PID_REGEX: Regex = Regex::new("^\\d{9}$").unwrap();
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "String")]
struct PassportID(String);

impl TryFrom<String> for PassportID {
    type Error = PassportParseError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        if PID_REGEX.is_match(&input) {
            Ok(Self(input))
        } else {
            Err(PassportParseError::BadStringFormat)
        }
    }
}

#[derive(Debug, Deserialize)]
struct PassportInfo {
    byr: BirthYear,
    iyr: IssueYear,
    eyr: ExpiryYear,
    hgt: Height,
    hcl: HairColor,
    ecl: EyeColor,
    pid: PassportID,
    cid: Option<String>,
}

pub fn good_passports_in(passport_json: &[String]) -> usize {
    passport_json
        .iter()
        .map(|pp| serde_json::from_str::<PassportInfo>(pp))
        .filter(|parse_result| parse_result.is_ok())
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    static VALID: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022";

    #[test]
    fn test_valid_passports() {
        let passport_json = make_json_strings(VALID);

        for pp in passport_json {
            let _: PassportInfo = serde_json::from_str(&pp).unwrap();
        }
    }
}
