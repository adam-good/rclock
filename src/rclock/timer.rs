use chrono::{DateTime, Local, TimeDelta, Utc};

use std::fmt;
use std::io;

pub struct Timer {
    last_update: DateTime<Local>,
    target: TimeDelta,
    delta: TimeDelta,
    state: TimerState,
}

#[derive(Eq, PartialEq)]
enum TimerState {
    Running,
    Paused,
}

impl Timer {
    pub fn new(target: TimeDelta) -> Self {
        Timer {
            last_update: Local::now(),
            target: target,
            delta: target,
            state: TimerState::Paused,
        }
    }

    /*
    pub fn from(mins: i64, secs: i64) -> Self {
        let delta = match TimeDelta::new(mins * 60 + secs, 0) {
            Some(t) => t,
            None => panic!("Invalid Timer Input {mins}:{secs}"),
        };
        Timer::new(delta)
    }
    */

    pub fn run(&mut self) {
        self.last_update = Local::now();
        self.state = TimerState::Running;
    }

    pub fn pause(&mut self) {
        self.state = TimerState::Paused;
    }

    pub fn update(&mut self) -> io::Result<()> {
        // TODO: Is there better syntax for this?
        match self.state {
            TimerState::Running => {
                let update_time: DateTime<Local> = Local::now();
                let offset: TimeDelta = update_time - self.last_update;
                self.delta = self.delta - offset;
                self.last_update = update_time;
                Ok(())
            }
            TimerState::Paused => Ok(()),
        }
    }

    pub fn time(&self) -> DateTime<Utc> {
        DateTime::<Utc>::default() + self.delta
    }

    pub fn get_perc(&self) -> f32 {
        let delta = self.delta.as_seconds_f32();
        let target = self.target.as_seconds_f32();

        (1.0 - (delta / target)) * 100.0
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self.time().format("%H:%M:%S")))
    }
}
