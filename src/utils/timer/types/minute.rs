
use std::fmt::Display;

pub const SECS_PER_MINUTE: u64 = 60;

#[derive(Debug, PartialEq)]
pub struct Minute {
    val: u64
}

impl Display for Minute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.val)
    }
}

impl Minute {
    pub fn new(val: u64) -> Self {
        if val > 60 {
            panic!("Minute-overflow");
        }

        Self { val }
    }

    pub fn from_secs(secs: u64) -> Self {
        Minute::new(secs / SECS_PER_MINUTE)
    }
}

#[cfg(test)]
mod tests {

use super::*;

    #[test]
    fn test_new() {
        let val = 5;
        let minute = Minute { val: val };
        assert_eq!(minute.val, val);
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

        assert_eq!(minutes.val, 3);
    }

    #[test]
    fn test_display() {
        let minutes = Minute { val: 5 };
        let result = minutes.to_string();
        let target = "05";
        assert_eq!(result, target);

        let hours = Minute { val: 12 };
        let result = hours.to_string();
        let target = "12";
        assert_eq!(result, target);
    }
}
