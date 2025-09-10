use chrono::DateTime;
use chrono::Local;
use std::io;

use std::fmt;

use crate::app::timer;

pub struct App {
    pub base_time: DateTime<Local>,
    timers: Vec<timer::Timer>,
}

impl App {
    pub fn new() -> Self {
        App {
            base_time: Local::now(),
            timers: Vec::<timer::Timer>::new(),
        }
    }

    pub fn new_timer(&mut self) -> io::Result<()> {
        self.timers.push(timer::Timer::new(Local::now()));
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
        match write!(f, "{}\n", self.base_time.format("%H:%M")) {
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
