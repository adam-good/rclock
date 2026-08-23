
mod timestamp;
mod time_provider;
use timestamp::{Timestamp};
use time_provider::{TimeProviderExt, TimeProvider, RealTimeProvider};
use std::time;
use std::fmt::Display;

pub type Timer = GenericTimer<RealTimeProvider>;

#[derive(Debug)]
pub struct GenericTimer<T: TimeProviderExt> {
    time_provider: T, 
    last_update: time::Instant,
    duration: time::Duration,
    run_state: RunState,
    timer_state: TimerState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum RunState {
    Running,
    Paused,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum TimerState {
    Active,
    Expired,
}

impl<T: TimeProviderExt> Display for GenericTimer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let timestamp = self.get_timestamp();
        write!(f, "{}", timestamp)
    }
}

impl GenericTimer<RealTimeProvider> {
    pub fn new(duration: time::Duration) -> Self {
        let mut provider = RealTimeProvider;
        let now = provider.now();
        GenericTimer {
            time_provider: provider,
            last_update: now,
            duration: duration,
            run_state: RunState::Paused,
            timer_state: TimerState::Active
        }
    } 
}

impl<T: TimeProviderExt> GenericTimer<T> {
    pub fn get_timestamp(&self) -> Timestamp {
        Timestamp::from_secs(self.duration.as_secs())
    }

    pub fn run(self) -> GenericTimer<T> {
        GenericTimer {
            time_provider: self.time_provider,
            last_update: self.last_update,
            duration: self.duration,
            run_state: RunState::Running,
            timer_state: self.timer_state
        }
    }

    pub fn pause(self) -> GenericTimer<T> {
        GenericTimer {
            time_provider: self.time_provider,
            last_update: self.last_update,
            duration: self.duration,
            run_state: RunState::Paused,
            timer_state: self.timer_state
        }
    }

    pub fn tick(&self) -> GenericTimer<T> {
        let mut time_provider = self.time_provider;
        let now = time_provider.now();
        let delta = now - self.last_update;
        let delta = if delta <= self.duration { self.duration } else { delta };
        match self.run_state { 
            RunState::Paused => GenericTimer {
                time_provider: time_provider,
                last_update: now,
                duration: self.duration,
                run_state: self.run_state,
                timer_state: self.timer_state
            },
            RunState::Running => {
                let diff  = self.duration - delta;
                let state = if diff.as_secs() > 0 { TimerState::Active  }
                            else { TimerState::Expired };
                GenericTimer {
                    time_provider: time_provider,
                    last_update: now, 
                    duration: diff, 
                    run_state: self.run_state,
                    timer_state: state
                }
            },
        }
    }
}

// Unit Tests
#[cfg(test)]
mod timer_tests {
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

    #[derive(Clone, Copy, Debug)]
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
    impl TimeProviderExt for MockTimeProvider {}

    #[test]
    fn test_new() {
        let dur = time::Duration::new(TOTAL_SECS, 0);
        let timer = GenericTimer::new(dur);
        assert_eq!(timer.duration, dur);
        assert_eq!(timer.run_state, RunState::Paused);
    }

    #[test]
    fn test_display() {
        let now = time::Instant::now();
        let time_prov = MockTimeProvider::new(now);
        let dur = time::Duration::new(TOTAL_SECS, 0);
        let timer = GenericTimer {
            time_provider: time_prov,
            last_update: now,
            duration: dur,
            run_state: RunState::Paused,
            timer_state: TimerState::Active,
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
        let timer = GenericTimer {
            time_provider: time_prov,
            last_update: now,
            duration: duration,
            run_state: RunState::Paused,
            timer_state: TimerState::Active,
        };
        let timer = timer.run();

        assert_eq!(timer.run_state, RunState::Running);
    }

    #[test]
    fn test_pause() {
        let now = time::Instant::now();
        let time_prov = MockTimeProvider::new(now);
        let duration = time::Duration::new(TOTAL_SECS, 0);
        let timer = GenericTimer {
            time_provider: time_prov,
            last_update: now,
            duration: duration,
            run_state: RunState::Running,
            timer_state: TimerState::Active,
        };
        let timer = timer.pause();

        assert_eq!(timer.run_state, RunState::Paused);
    }

    #[test]
    fn test_get_timestamp() {
        let now = time::Instant::now();
        let time_prov = MockTimeProvider::new(now);
        let dur = time::Duration::new(TOTAL_SECS, 0);
        let timer = GenericTimer {
            time_provider: time_prov,
            last_update: now,
            duration: dur,
            run_state: RunState::Paused,
            timer_state: TimerState::Active,
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
        let timer = GenericTimer {
            time_provider: time_prov,
            last_update: now,
            duration: dur, 
            run_state: RunState::Running,
            timer_state: TimerState::Active,
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
        let timer = GenericTimer {
            time_provider: time_prov,
            last_update: now,
            duration: dur,
            run_state: RunState::Paused,
            timer_state: TimerState::Active,
        };

        let timer = timer.tick();

        let result = timer.duration;

        assert_eq!(result.as_secs(), dur.as_secs());
    }

    #[test]
    fn test_tick_exire() {
        let now = time::Instant::now();
        let time_prov = MockTimeProvider::new(now);
        let dur = time::Duration::new(0, 0);
        let timer = GenericTimer {
            time_provider: time_prov,
            last_update: now,
            duration: dur,
            run_state: RunState::Running,
            timer_state: TimerState::Active,
        };

        let timer = timer.tick();
        let result = timer.timer_state;

        assert_eq!(result, TimerState::Expired);
    }
}
