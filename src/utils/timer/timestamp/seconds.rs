
use crate::utils::timer::timestamp::constants::{SECS_PER_HOUR, SECS_PER_MINUTE};
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Second {
    pub value: u64
}

impl Display for Second {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.value)
    }
}

impl Second {
    pub fn new(val: u64) -> Self {
        if val > 60 {
            panic!("Second-overflow");
        }

        Self { value: val }
    }

    pub fn from_secs(secs: u64) -> Self {
        let secs = secs % SECS_PER_HOUR;
        let secs = secs % SECS_PER_MINUTE;
        Self::new(secs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_new() {
        let val = 5;
        let second = Second { value: val };
        assert_eq!(second.value, val);
    }

    #[test]
    #[should_panic(expected = "Second-overflow")]
    fn test_overflow_panic() {
        Second::new(61); // Should Panic
    }
}
