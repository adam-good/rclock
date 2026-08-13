
use std::time;

#[derive(Debug)]
pub struct Timer {
    start: time::Instant,
    duration: time::Duration
}

impl Timer {
    pub fn new(start: time::Instant) -> Timer {
        Timer { start: start, duration: time::Duration::new(0, 0) }
    }

    pub fn from_now() -> Timer {
        Timer::new(time::Instant::now())
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
        let inst = time::Instant::now();
        let timer = Timer::new(inst);
        assert_eq!(timer.start, inst);
        assert_eq!(timer.duration, inst-inst);
    }

    #[test]
    fn test_tick() {
        let dur = time::Duration::new(2, 0);
        let timer = Timer {
            start: time::Instant::now(),
            duration: time::Duration::new(0, 0), 
        };

        std::thread::sleep(dur);
        let timer = timer.tick();


        assert_eq!(timer.duration.as_secs(), dur.as_secs());
    }
}
