use chrono::DateTime;
use chrono::Local;
use chrono::TimeDelta;
use chrono::Utc;

use std::fmt;
use std::io;

pub struct Timer {
    last_update: DateTime<Local>,
    target: TimeDelta,
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
                self.target = self.target - offset;
                self.last_update = update_time;
                Ok(())
            }
            TimerState::Paused => Ok(()),
        }
    }

    pub fn time(&self) -> DateTime<Utc> {
        DateTime::<Utc>::default() + self.target
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", (self.time().format("%H:%M:%S")))
    }
}
