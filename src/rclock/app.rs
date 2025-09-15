use chrono::DateTime;
use chrono::Local;
use std::io;

use std::fmt;

use crate::rclock::timer;

pub struct App {
    pub base_time: DateTime<Local>,
    primary_timer_idx: Option<usize>,
    timers: Vec<timer::Timer>,
}

impl App {
    pub fn new() -> Self {
        App {
            base_time: Local::now(),
            primary_timer_idx: None,
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

        if self.primary_timer_idx.is_none() && !self.timers.is_empty() {
            self.primary_timer_idx = Some(0);
        }

        Ok(())
    }

    pub fn get_primary_timer(&self) -> Option<&timer::Timer> {
        match self.primary_timer_idx {
            Some(i) => self.timers.get(i),
            None => None,
        }
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
