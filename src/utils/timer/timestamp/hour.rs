

use crate::utils::timer::timestamp::constants::{SECS_PER_HOUR};
use std::fmt::Display;


#[derive(Debug, PartialEq)]
pub struct Hour {
    pub value: u64
}

impl Display for Hour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.value)
    }
}

impl Hour {
    pub fn new(val: u64) -> Self {
        if val > 24 {
            panic!("Hour-overflow");
        }

        Self { value: val }
    }

    pub fn from_secs(secs: u64) -> Self {
        Hour::new(secs / SECS_PER_HOUR)
    }
}

#[cfg(test)]
mod tests {

use super::*;

    #[test]
    fn test_new() {
        let val = 5;
        let hour = Hour::new(val);
        assert_eq!(hour.value, val);
    }

    #[test]
    #[should_panic(expected = "Hour-overflow")]
    fn test_overflow_panic() {
        Hour::new(25); // Should Panic
    }

    #[test]
    fn test_from_secs() {
        let secs = SECS_PER_HOUR * 3 + 32;
        let hours = Hour::from_secs(secs);

        assert_eq!(hours.value, 3);
    }

    #[test]
    fn test_display() {
        let hours = Hour { value: 5 };
        let result = hours.to_string();
        let target = "05";
        assert_eq!(result, target);

        let hours = Hour { value: 12 };
        let result = hours.to_string();
        let target = "12";
        assert_eq!(result, target);
    }
}
