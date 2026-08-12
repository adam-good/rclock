
use std::time;

#[derive(Debug)]
pub struct Timer {
    start: time::Instant,
    duration: time::Duration
}

impl Timer {
    pub fn new() -> Timer {
        let now = time::Instant::now();
        Timer { start: now, duration: now-now }
    }

    pub fn tick(self) -> Timer {
        let now = time::Instant::now();
        Timer { start: self.start, duration: now-self.start }
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let timer = Timer::new();
        assert_eq!(timer.start, timer.duration)
    }
}
