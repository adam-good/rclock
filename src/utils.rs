
use std::time;

#[derive(Debug)]
pub struct Timer {
    base_time: time::Instant,
    tick_time: time::Instant,
    state: TimerState,
}


#[derive(Debug)]
enum TimerState {
    Running,
    Paused,
}

impl Timer {
    pub fn new(start: time::Instant) -> Timer {
        Timer { 
            base_time: start,
            tick_time: start,
            state: TimerState::Paused
        }
    }

    pub fn from_now() -> Timer {
        Timer::new(time::Instant::now())
    }

    fn shift(self, delta: time::Duration) -> Timer {
        Timer { 
            base_time: self.base_time + delta, 
            tick_time: self.tick_time + delta, 
            state: self.state 
        }
    }

    fn advance(self, delta: time::Duration) -> Timer {
        Timer { 
            base_time: self.base_time, 
            tick_time: self.tick_time + delta, 
            state: self.state 
        }
    }

    pub fn tick(self) -> Timer {
        let delta = time::Instant::now() - self.tick_time;
        match self.state { 
            TimerState::Paused => self.shift(delta),
            TimerState::Running => self.advance(delta),
        }
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
        assert_eq!(timer.base_time, inst);
        assert_eq!(timer.tick_time, inst);
    }

    #[test]
    fn test_tick_running() {
        let dur = time::Duration::new(2, 0);
        let now = time::Instant::now();
        let timer = Timer {
            base_time: now,
            tick_time: now,
            state: TimerState::Running,
        };

        std::thread::sleep(dur);
        let timer = timer.tick();
        let delta = timer.tick_time - timer.base_time;

        assert_eq!(delta.as_secs(), dur.as_secs());
    }

    #[test]
    fn test_tick_paused() {
        let dur = time::Duration::new(2, 0);
        let now = time::Instant::now();
        let timer = Timer {
            base_time: now,
            tick_time: now,
            state: TimerState::Paused,
        };

        std::thread::sleep(dur);
        let timer = timer.tick();
        let delta = timer.tick_time - timer.base_time;

        assert_eq!(delta.as_secs(), 0 );
    }
}
