use crate::rclock::timer;
use chrono::TimeDelta;

pub struct Pomodoro {
    timer: timer::Timer,
    work_time: TimeDelta,
    short_break_time: TimeDelta,
    long_break_time: TimeDelta,
    round_counter: i32,
}

impl Pomodoro {
    pub fn new(work: i64, short_break: i64, long_break: i64) -> Self {
        let work_delta: TimeDelta =
            TimeDelta::new(work * 60, 0).expect("Failed to Create Work Timer {work}");
        let break_delta1: TimeDelta = TimeDelta::new(short_break * 60, 0)
            .expect("Failed to Create Short Break Timer {short_break}");
        let break_delta2: TimeDelta = TimeDelta::new(long_break * 60, 0)
            .expect("Failed to Create Long Break Timer {long_break}");

        Self {
            timer: timer::Timer::new(work_delta),
            work_time: work_delta,
            short_break_time: break_delta1,
            long_break_time: break_delta2,
            round_counter: 0,
        }
    }
}
