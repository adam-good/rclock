
mod timestamp;
use timestamp::{Timestamp};
use std::time;
use std::fmt::Display;

pub trait TimeProvider {
    fn now(&mut self) -> std::time::Instant;
}

#[derive(Clone, Copy)]
pub struct RealTimeProvider;

impl TimeProvider for RealTimeProvider {
    fn now(&mut self) -> std::time::Instant {
        std::time::Instant::now()
    }
}

#[derive(Debug)]
pub struct Timer<T: TimeProvider> {
    time_provider: T, 
    last_update: time::Instant,
    duration: time::Duration,
    state: TimerState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TimerState {
    Running,
    Paused,
}

impl<T: TimeProvider> Display for Timer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let timestamp = self.get_timestamp();
        write!(f, "{}", timestamp)
    }
}

impl<T: TimeProvider> Timer<T> {
    pub fn new(mut time_provider: T, duration: time::Duration) -> Timer<T> {
        let now = time_provider.now();
        Timer {
            time_provider: time_provider,
            last_update: now,
            duration: duration,
            state: TimerState::Paused,
        }
    }

    pub fn get_timestamp(&self) -> Timestamp {
        Timestamp::from_secs(self.duration.as_secs())
    }

    pub fn run(self) -> Timer<T> {
        Timer {
            time_provider: self.time_provider,
            last_update: self.last_update,
            duration: self.duration,
            state: TimerState::Running,
        }
    }

    pub fn pause(self) -> Timer<T> {
        Timer {
            time_provider: self.time_provider,
            last_update: self.last_update,
            duration: self.duration,
            state: TimerState::Paused, 
        }
    }

    pub fn tick(self) -> Timer<T> {
        let mut time_provider = self.time_provider;
        let now = time_provider.now();
        let delta = now - self.last_update;
        match self.state { 
            TimerState::Paused => Timer {
                time_provider: time_provider,
                last_update: now,
                duration: self.duration,
                state: self.state
            },
            TimerState::Running => Timer {
                time_provider: time_provider,
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

    struct MockTimeProvider {
        init_time: time::Instant,
        ticks: u64,
    }

    impl MockTimeProvider {
        fn new(init_time: time::Instant) -> Self {
            MockTimeProvider { init_time: init_time, ticks: 0 }
        }
    }
    impl TimeProvider for MockTimeProvider {
        fn now(&mut self) -> std::time::Instant {
            self.ticks += 1;
            self.init_time + time::Duration::new(self.ticks, 0)
        }
    }

    #[test]
    fn test_new() {
        let now = time::Instant::now();
        let time_provider = MockTimeProvider::new(now);
        let dur = time::Duration::new(TOTAL_SECS, 0);
        let timer = Timer::new(time_provider, dur);
        assert_eq!(timer.duration, dur);
        assert_eq!(timer.state, TimerState::Paused);
    }

    #[test]
    fn test_display() {
        let now = time::Instant::now();
        let time_prov = MockTimeProvider::new(now);
        let dur = time::Duration::new(TOTAL_SECS, 0);
        let timer = Timer {
            time_provider: time_prov,
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
        let time_prov = MockTimeProvider::new(now);
        let duration = time::Duration::new(TOTAL_SECS, 0);
        let timer = Timer {
            time_provider: time_prov,
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
        let time_prov = MockTimeProvider::new(now);
        let duration = time::Duration::new(TOTAL_SECS, 0);
        let timer = Timer {
            time_provider: time_prov,
            last_update: now,
            duration: duration,
            state: TimerState::Running,
        };
        let timer = timer.pause();

        assert_eq!(timer.state, TimerState::Paused);
    }

    #[test]
    fn test_get_timestamp() {
        let now = time::Instant::now();
        let time_prov = MockTimeProvider::new(now);
        let dur = time::Duration::new(TOTAL_SECS, 0);
        let timer = Timer {
            time_provider: time_prov,
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
        let now = time::Instant::now();
        let time_prov = MockTimeProvider::new(now);
        let dur = time::Duration::new(TOTAL_SECS, 0);
        let target = time::Duration::new(TOTAL_SECS - 1, 0);
        let timer = Timer {
            time_provider: time_prov,
            last_update: now,
            duration: dur, 
            state: TimerState::Running,
        };

        let timer = timer.tick();

        let result = timer.duration;

        assert_eq!(target.as_secs(), result.as_secs());
    }

    #[test]
    fn test_tick_paused() {
        let now = time::Instant::now();
        let time_prov = MockTimeProvider::new(now);
        let dur = time::Duration::new(TOTAL_SECS, 0);
        let timer = Timer {
            time_provider: time_prov,
            last_update: now,
            duration: dur,
            state: TimerState::Paused,
        };

        let timer = timer.tick();

        let result = timer.duration;

        assert_eq!(result.as_secs(), dur.as_secs());
    }
}
