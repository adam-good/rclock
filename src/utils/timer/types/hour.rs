
use std::fmt::Display;

pub const SECS_PER_HOUR: u64 = 3600;

#[derive(Debug, PartialEq)]
pub struct Hour {
    val: u64
}

impl Display for Hour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.val)
    }
}

impl Hour {
    pub fn new(val: u64) -> Self {
        if val > 24 {
            panic!("Hour-overflow");
        }

        Self { val }
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
        assert_eq!(hour.val, val);
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

        assert_eq!(hours.val, 3);
    }

    #[test]
    fn test_display() {
        let hours = Hour { val: 5 };
        let result = hours.to_string();
        let target = "05";
        assert_eq!(result, target);

        let hours = Hour { val: 12 };
        let result = hours.to_string();
        let target = "12";
        assert_eq!(result, target);
    }
}
