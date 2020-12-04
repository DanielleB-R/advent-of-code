use custom_error::custom_error;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PasswordRule {
    min: usize,
    max: usize,
    letter: char,
}

lazy_static! {
    static ref RULE_REGEX: Regex = Regex::new("^(\\d+)-(\\d+) (\\w)$").unwrap();
}

custom_error! {pub PasswordParseError
               IncorrectPattern = "input does not match the expected pattern"
}

impl FromStr for PasswordRule {
    type Err = PasswordParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RULE_REGEX
            .captures(s)
            .ok_or(PasswordParseError::IncorrectPattern)?;

        // We can unwrap things because everything should be good
        // after the regex match.
        let min = captures.get(1).unwrap().as_str().parse().unwrap();
        let max = captures.get(2).unwrap().as_str().parse().unwrap();
        let letter = captures.get(3).unwrap().as_str().chars().next().unwrap();

        Ok(PasswordRule { min, max, letter })
    }
}

impl PasswordRule {
    pub fn matches(&self, password: &str) -> bool {
        let count = password.chars().filter(|&c| c == self.letter).count();

        count >= self.min && count <= self.max
    }

    pub fn matches_new_logic(&self, password: &str) -> bool {
        // NOTE: I'm assuming ASCII here
        let first_char = password.as_bytes()[self.min - 1] as char;
        let second_char = password.as_bytes()[self.max - 1] as char;

        (first_char != second_char) && (first_char == self.letter || second_char == self.letter)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsing_rules() {
        assert_eq!(
            "1-3 a".parse::<PasswordRule>().unwrap(),
            PasswordRule {
                min: 1,
                max: 3,
                letter: 'a'
            }
        );

        assert_eq!(
            "1-3 b".parse::<PasswordRule>().unwrap(),
            PasswordRule {
                min: 1,
                max: 3,
                letter: 'b'
            }
        );

        assert_eq!(
            "2-12 c".parse::<PasswordRule>().unwrap(),
            PasswordRule {
                min: 2,
                max: 12,
                letter: 'c'
            }
        );
    }

    #[test]
    fn test_evaluating_rules() {
        let rule = PasswordRule {
            min: 3,
            max: 8,
            letter: 'x',
        };

        assert!(rule.matches("axbxcx"));
        assert!(rule.matches("xxxxxxxx"));
        assert!(!rule.matches("axe"));
    }

    #[test]
    fn test_new_style_logic() {
        assert!("1-3 a"
            .parse::<PasswordRule>()
            .unwrap()
            .matches_new_logic("abcde"));
        assert!(!"1-3 b"
            .parse::<PasswordRule>()
            .unwrap()
            .matches_new_logic("cdefg"));
        assert!(!"2-9 c"
            .parse::<PasswordRule>()
            .unwrap()
            .matches_new_logic("ccccccccc"));
    }
}
