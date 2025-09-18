use crate::rclock::timer;
use chrono::TimeDelta;
use std::fmt;

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

    pub fn get_timer(&self) -> &timer::Timer {
        &self.timer
    }

    pub fn get_round(&self) -> i32 {
        self.round_counter
    }

    pub fn update(&mut self) {
        self.timer.update();
    }
}

impl fmt::Display for Pomodoro {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match write!(
            f,
            "{{timer: {}\nwork: {}\nbreak1: {}\nbreak2: {}\nround: {}}}",
            self.timer,
            self.work_time,
            self.short_break_time,
            self.long_break_time,
            self.round_counter
        ) {
            Ok(_v) => {}
            Err(_e) => panic!("Error Printing Pomodoro"),
        };

        Ok(())
    }
}
