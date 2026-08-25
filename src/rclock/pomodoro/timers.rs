
use std::time::Duration;
use crate::utils::timer::{Timer, TimerState};

#[derive(Debug,PartialEq,Clone,Copy)]
pub enum PomoType {
    Work,
    Rest,
}

enum PomoState {
    Running,
    Paused,
}


pub struct PomoRecord {
    duration: Duration,
    pomo_type: PomoType,
}

impl PomoRecord {
    pub fn new(duration: Duration, pomo_type: PomoType) -> Self {
        PomoRecord { duration, pomo_type }
    }
    pub fn new_work(duration: Duration) -> Self {
        PomoRecord::new(duration, PomoType::Work)
    }
    pub fn new_rest(duration: Duration) -> Self {
        PomoRecord::new(duration, PomoType::Rest)
    }
}

pub struct PomoTimer {
    timer: Timer,
    pomo_type: PomoType,
    state: PomoState,
}

impl PomoTimer {
    pub fn new(duration: Duration, pomo_type: PomoType) -> Self {
        Self { timer: Timer::new(duration), pomo_type, state: PomoState::Paused }
    }
    pub fn new_work(duration: Duration) -> Self {
        Self::new(duration, PomoType::Work)
    }
    pub fn new_rest(duration: Duration) -> Self {
        Self::new(duration, PomoType::Rest)
    }

    pub fn from_record(record: &PomoRecord) -> Self {
        PomoTimer::new(record.duration, record.pomo_type)
    }

    pub fn get_type(&self) -> PomoType {
        self.pomo_type
    }

    pub fn run(self) -> Self {
        Self { 
            timer: self.timer.run(), 
            pomo_type: self.pomo_type, 
            state: PomoState::Running 
        }
    }

    pub fn pause(self) -> Self {
        Self { 
            timer: self.timer.pause(),
            pomo_type: self.pomo_type, 
            state: PomoState::Paused 
        }
    }

    pub fn update(self) -> Self {
        Self {
            timer: self.timer.tick(),
            pomo_type: self.pomo_type,
            state: self.state
        }
    }

    pub fn is_finished(&self) -> bool {
        self.timer.get_state() == TimerState::Expired
    }
}

#[cfg(test)]
mod pomo_timer_tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_new() {
        let duration = Duration::new(10, 0);
        let pomo_type = PomoType::Work;
        let pomo_timer = PomoTimer::new(duration, pomo_type);

        let target_timer = Timer::new(duration);
        let target_type  = PomoType::Work;

        assert_eq!(pomo_timer.timer.get_duration(), target_timer.get_duration()); 
        assert_eq!(pomo_timer.pomo_type, target_type);
    }

    #[test]
    fn test_new_work() {
        let duration = Duration::new(10, 0);
        let timer = PomoTimer::new_work(duration);

        assert_eq!(timer.pomo_type, PomoType::Work);
    }

    #[test]
    fn test_new_rest() {
        let duration = Duration::new(10, 0);
        let timer = PomoTimer::new_rest(duration);

        assert_eq!(timer.pomo_type, PomoType::Rest);
    }

    #[test]
    fn test_from_record() {
        let duration = Duration::new(10, 0);
        let pomo_type = PomoType::Work;
        let record = PomoRecord::new(duration, pomo_type);
        let pomo_timer = PomoTimer::from_record(&record);

        assert_eq!(pomo_timer.timer.get_duration(), duration);
        assert_eq!(pomo_timer.pomo_type, pomo_type);
    }

    #[test]
    fn test_get_type() {
        let duration = Duration::new(10, 0);
        let pomo_timer = PomoTimer {
            timer: Timer::new(duration),
            pomo_type: PomoType::Work,
            state: PomoState::Running
        };

        let result = pomo_timer.get_type();
        let target = PomoType::Work;

        assert_eq!(result, target);
    }
}
