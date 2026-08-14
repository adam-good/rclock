
use crate::utils::timer::timestamp::constants::{SECS_PER_HOUR, SECS_PER_MINUTE};
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Minute {
    pub value: u64
}

impl Display for Minute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.value)
    }
}

impl Minute {
    pub fn new(val: u64) -> Self {
        if val > 60 {
            panic!("Minute-overflow");
        }

        Self { value: val }
    }

    pub fn from_secs(secs: u64) -> Self {
        let secs = secs % SECS_PER_HOUR;
        Minute::new(secs / SECS_PER_MINUTE)
    }
}

#[cfg(test)]
mod tests {

use super::*;

    #[test]
    fn test_new() {
        let val = 5;
        let minute = Minute { value: val };
        assert_eq!(minute.value, val);
    }

    #[test]
    #[should_panic(expected = "Minute-overflow")]
    fn test_overflow_panic() {
        Minute::new(61); // Should Panic
    }

    #[test]
    fn test_from_secs() {
        let secs = SECS_PER_MINUTE * 3 + 32;
        let minutes = Minute::from_secs(secs);

        assert_eq!(minutes.value, 3);
    }

    #[test]
    fn test_display() {
        let minutes = Minute { value: 5 };
        let result = minutes.to_string();
        let target = "05";
        assert_eq!(result, target);

        let hours = Minute { value: 12 };
        let result = hours.to_string();
        let target = "12";
        assert_eq!(result, target);
    }
}
