use chrono::DateTime;
use chrono::Local;
use chrono::TimeDelta;

use std::fmt;

pub struct Timer {
    start_time: DateTime<Local>,
    offset: TimeDelta,
}
impl Timer {
    pub fn new(start: DateTime<Local>) -> Self {
        Timer {
            start_time: start,
            offset: TimeDelta::new(0, 0).unwrap(),
        }
    }
    pub fn update(&mut self) {
        let new_offset = Local::now() - self.start_time;

        self.offset = new_offset;
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.offset)
    }
}
