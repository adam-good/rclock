
mod timestamp;
use timestamp::{Timestamp};
use std::time;
use std::fmt::Display;

#[derive(Debug)]
pub struct Timer {
    last_update: time::Instant,
    duration: time::Duration,
    state: TimerState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TimerState {
    Running,
    Paused,
}

impl Display for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let timestamp = self.get_timestamp();
        write!(f, "{}", timestamp)
    }
}

impl Timer {
    pub fn new(duration: time::Duration) -> Timer {
        Timer {
            last_update: time::Instant::now(),
            duration: duration,
            state: TimerState::Paused,
        }
    }

    pub fn get_timestamp(&self) -> Timestamp {
        Timestamp::from_secs(self.duration.as_secs())
    }

    pub fn run(self) -> Timer {
        Timer {
            last_update: self.last_update,
            duration: self.duration,
            state: TimerState::Running,
        }
    }

    pub fn pause(self) -> Timer {
        Timer {
            last_update: self.last_update,
            duration: self.duration,
            state: TimerState::Paused, 
        }
    }

    pub fn tick(&self) -> Timer {
        let now = time::Instant::now();
        let delta = now - self.last_update;
        match self.state { 
            TimerState::Paused => Timer {
                last_update: now,
                duration: self.duration,
                state: self.state
            },
            TimerState::Running => Timer {
                last_update: now, 
                duration: self.duration - delta,
                state: self.state 
            },
        }
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    // TODO: Import these from constants
    const SECS_PER_MINUTE: u64 = 60;
    const SECS_PER_HOUR:   u64 = 3600;

    // NOTE: For use across multiple tests
    const HOURS:   u64 = 14;
    const MINUTES: u64 = 6;
    const SECONDS: u64 = 32;
    const TOTAL_SECS: u64 = (HOURS * SECS_PER_HOUR) + 
                            (MINUTES * SECS_PER_MINUTE) +
                            SECONDS;

    #[test]
    fn test_new() {
        let dur = time::Duration::new(TOTAL_SECS, 0);
        let timer = Timer::new(dur);
        assert_eq!(timer.duration, dur);
        assert_eq!(timer.state, TimerState::Paused);
    }

    #[test]
    fn test_display() {
        let dur = time::Duration::new(TOTAL_SECS, 0);
        let now = time::Instant::now();
        let timer = Timer {
            last_update: now,
            duration: dur,
            state: TimerState::Paused
        };

        let restult = timer.to_string();
        let target = "14:06:32";

        assert_eq!(restult, target);
    }

    #[test]
    fn test_run() {
        let now = time::Instant::now();
        let duration = time::Duration::new(TOTAL_SECS, 0);
        let timer = Timer {
            last_update: now,
            duration: duration,
            state: TimerState::Paused,
        };
        let timer = timer.run();

        assert_eq!(timer.state, TimerState::Running);
    }

    #[test]
    fn test_pause() {
        let now = time::Instant::now();
        let duration = time::Duration::new(TOTAL_SECS, 0);
        let timer = Timer {
            last_update: now,
            duration: duration,
            state: TimerState::Running,
        };
        let timer = timer.pause();

        assert_eq!(timer.state, TimerState::Paused);
    }

    #[test]
    fn test_get_timestamp() {
        let dur = time::Duration::new(TOTAL_SECS, 0);
        let now = time::Instant::now();
        let timer = Timer {
           last_update: now,
           duration: dur,
           state: TimerState::Paused,
        };

        let result = timer.get_timestamp();
        let target = Timestamp::new(HOURS, MINUTES, SECONDS);

        assert_eq!(result, target)
    }

    #[test]
    fn test_tick_running() {
        let dur = time::Duration::new(TOTAL_SECS, 0);
        let update_time = time::Instant::now() - time::Duration::new(1, 0);
        let timer = Timer {
            last_update: update_time,
            duration: dur, 
            state: TimerState::Running,
        };

        // TODO: Implement a Mock Interface for Time 
//        std::thread::sleep(dur);
//        let timer = timer.tick();
//        let delta = timer.tick_time - timer.base_time;
//
//        assert_eq!(delta.as_secs(), dur.as_secs());
    }

    #[test]
    fn test_tick_paused() {
        let dur = time::Duration::new(2, 0);
        let now = time::Instant::now();
        let timer = Timer {
            last_update: now,
            duration: dur,
            state: TimerState::Paused,
        };

        // TODO: Implement a Mock Interface for Time 
  //      std::thread::sleep(dur);
  //      let timer = timer.tick();
  //      let delta = timer.tick_time - timer.base_time;

  //      assert_eq!(delta.as_secs(), 0 );
    }
}
