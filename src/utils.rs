
use std::time;

#[derive(Debug)]
pub struct Timer {
    start: time::Instant
}

impl Timer {
    pub fn new() -> Timer {
        Timer { start: time::Instant::now() }
    }
}
