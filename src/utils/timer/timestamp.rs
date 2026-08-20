
mod hour;
mod minute;
mod seconds;
mod constants;
pub use hour::Hour;
pub use minute::Minute;
pub use seconds::Second;
use constants::{SECS_PER_HOUR, SECS_PER_MINUTE};

use std::{fmt::Display};

#[derive(PartialEq, Debug)]
pub struct Timestamp {
    hour: Hour,
    minute: Minute,
    second: Second,
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}:{:02}",
            self.hour.value,
            self.minute.value,
            self.second.value)
    }
}

impl Timestamp {
    pub fn new(hour: u64, minute: u64, second: u64) -> Self {
        Self { 
            hour: Hour::new(hour), 
            minute: Minute::new(minute), 
            second: Second::new(second) }
    }

    pub fn from_secs(secs: u64) -> Self {
        Self { 
            hour:   Hour::from_secs(secs), 
            minute: Minute::from_secs(secs), 
            second: Second::from_secs(secs) 
        }
    }

    pub fn to_secs(self) -> u64 {
        self.hour * SECS_PER_HOUR + self.minute * SECS_PER_MINUTE + self.second 
    }
}
