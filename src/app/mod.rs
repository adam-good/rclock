use chrono::DateTime;
use chrono::Local;
use chrono::TimeDelta;

use std::io;

use std::fmt;

pub struct App {
    pub base_time: DateTime<Local>,
    timers: Vec<Timer>,
}

struct Timer {
    start_time: DateTime<Local>,
    offset: TimeDelta,
}

impl App {
    pub fn new() -> Self {
        App {
            base_time: Local::now(),
            timers: Vec::<Timer>::new(),
        }
    }

    pub fn new_timer(&mut self) -> io::Result<()> {
        self.timers.push(Timer::new(self.base_time));
        Ok(())
    }

    pub fn update(&mut self) -> io::Result<()> {
        self.base_time = Local::now();
        for t in &mut self.timers {
            t.update();
        }

        Ok(())
    }
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match write!(f, "{}\n", self.base_time) {
            Ok(v) => v,
            Err(_e) => panic!("Error Displaying App"),
        }
        for t in &self.timers {
            match write!(f, "  {}\n", t) {
                Ok(v) => v,
                Err(_e) => panic!("Error Displaying Timer"),
            }
        }
        Ok(())
    }
}

impl Timer {
    fn new(start: DateTime<Local>) -> Self {
        Timer {
            start_time: start,
            offset: TimeDelta::new(0, 0).unwrap(),
        }
    }
    fn update(&mut self) {
        let new_offset = Local::now() - self.start_time;

        self.offset = new_offset;
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.offset)
    }
}
