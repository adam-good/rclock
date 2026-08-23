use chrono::{DateTime, Local, TimeDelta, Utc};

use std::fmt;
use std::io;

pub struct _Timer {
    last_update: DateTime<Local>,
    target: TimeDelta,
    delta: TimeDelta,
    state: _TimerState,
}

#[derive(Eq, PartialEq)]
enum _TimerState {
    Running,
    Paused,
}

impl _Timer {
    pub fn _new(target: TimeDelta) -> Self {
        _Timer {
            last_update: Local::now(),
            target: target,
            delta: target,
            state: _TimerState::Paused,
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

    pub fn _run(&mut self) {
        self.last_update = Local::now();
        self.state = _TimerState::Running;
    }

    pub fn _pause(&mut self) {
        self.state = _TimerState::Paused;
    }

    pub fn _update(&mut self) -> io::Result<()> {
        // TODO: Is there better syntax for this?
        match self.state {
            _TimerState::Running => {
                let update_time: DateTime<Local> = Local::now();
                let offset: TimeDelta = update_time - self.last_update;
                self.delta = self.delta - offset;
                self.last_update = update_time;
                Ok(())
            }
            _TimerState::Paused => Ok(()),
        }
    }

    pub fn _time(&self) -> DateTime<Utc> {
        DateTime::<Utc>::default() + self.delta
    }

    pub fn _get_perc(&self) -> f32 {
        let delta = self.delta.as_seconds_f32();
        let target = self.target.as_seconds_f32();

        (1.0 - (delta / target)) * 100.0
    }
}

impl fmt::Display for _Timer {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
        //write!(f, "{}", (self.time().format("%H:%M:%S")))
    }
}
