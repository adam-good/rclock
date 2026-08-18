
use crate::rclock::pomodoro::timers::{PomoTimer, PomoRecord};
use std::iter::zip;
use std::time::Duration;

pub struct PomoSchedule {
    pomos: Vec<PomoRecord>,
    current: usize, 
}

impl PomoSchedule {
    pub fn new(pomos: Vec<PomoRecord>) -> Self {
        Self { pomos, current: 0 }
    }

    pub fn from_vecs(work_times: Vec<u64>, rest_times: Vec<u64>) -> Self {
        if work_times.len() != rest_times.len() {
            panic!("Failed to Create Schedules from Asymmetrical Work and Rest Times");
        }
        let records: Vec<PomoRecord> = zip(work_times, rest_times)
                      .map(|(w,r)| (Duration::new(w, 0), Duration::new(r, 0)))
                      .map(|(w,r)| [PomoRecord::new_work(w), PomoRecord::new_rest(r)])
                      .flatten()
                      .collect();

        PomoSchedule { pomos: records, current: 0 }
    }

    pub fn inc(self) -> Self {
        Self { pomos: self.pomos, current: self.current+1 }
    }

    fn map_index(&self) -> usize {
        self.current % self.pomos.len()
    }

    pub fn current_timer(&self) -> PomoTimer {
        let record = self.pomos.get(self.map_index())
                     .expect("Pomo Scheule Overflow");
        PomoTimer::from_record(record)
    }
 }
