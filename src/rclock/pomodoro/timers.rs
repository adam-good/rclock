
use std::time::Duration;
use rclock::utils::timer::Timer;

#[derive(Clone,Copy)]
enum PomoType {
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
}

