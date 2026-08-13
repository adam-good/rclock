
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Second {
    val: u64
}

impl Display for Second {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}", self.val)
    }
}

impl Second {
    pub fn new(val: u64) -> Self {
        if val > 60 {
            panic!("Second-overflow");
        }

        Self { val }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn test_new() {
        let val = 5;
        let second = Second { val: val };
        assert_eq!(second.val, val);
    }

    #[test]
    #[should_panic(expected = "Second-overflow")]
    fn test_overflow_panic() {
        Second::new(61); // Should Panic
    }
}
