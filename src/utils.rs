
use std::time;

#[derive(Debug)]
pub struct Timer {
    base_time: time::Instant,
    tick_time: time::Instant,
    state: TimerState,
}


#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn duration(self) -> time::Duration {
        self.tick_time - self.base_time
    }

    pub fn run(self) -> Timer {
        Timer { 
            base_time: self.base_time,
            tick_time: self.tick_time, 
            state: TimerState::Running 
        }
    }

    pub fn pause(self) -> Timer {
        Timer { 
            base_time: self.base_time, 
            tick_time: self.tick_time, 
            state: TimerState::Paused 
        }
    }

    fn shift(&self, delta: time::Duration) -> Timer {
        Timer { 
            base_time: self.base_time + delta, 
            tick_time: self.tick_time + delta, 
            state: self.state 
        }
    }

    fn advance(&self, delta: time::Duration) -> Timer {
        Timer { 
            base_time: self.base_time, 
            tick_time: self.tick_time + delta, 
            state: self.state 
        }
    }

    pub fn tick(&self) -> Timer {
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
    fn test_run() {
        let now = time::Instant::now();
        let timer = Timer {
            base_time: now,
            tick_time: now,
            state: TimerState::Paused,
        };
        let timer = timer.run();

        assert_eq!(timer.state, TimerState::Running);
    }

    #[test]
    fn test_pause() {
        let now = time::Instant::now();
        let timer = Timer {
            base_time: now,
            tick_time: now,
            state: TimerState::Running,
        };
        let timer = timer.pause();

        assert_eq!(timer.state, TimerState::Paused);
    }

    #[test]
    fn test_duration() {
        let dur = time::Duration::new(2, 0);
        let now = time::Instant::now();
        let timer = Timer {
            base_time: now,
            tick_time: now + dur,
            state: TimerState::Running,
        };

        assert_eq!(timer.duration(), dur);
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

        // TODO: do this without sleeping?
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

        // TODO: Do this without sleeping?
        std::thread::sleep(dur);
        let timer = timer.tick();
        let delta = timer.tick_time - timer.base_time;

        assert_eq!(delta.as_secs(), 0 );
    }
}
