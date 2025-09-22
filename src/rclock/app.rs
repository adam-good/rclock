use chrono::DateTime;
use chrono::Local;
use std::io;

use std::fmt;
use std::io::Error;

use crate::rclock::pomodoro;
use crate::rclock::pomodoro::Pomodoro;
use crate::rclock::timer;

pub struct App {
    pub base_time: DateTime<Local>,
    pomodoro: Option<pomodoro::Pomodoro>,
    state: AppState,
}

#[derive(Eq, PartialEq)]
enum AppState {
    Running, // App is fully running
    Stopped, // App is completely stopped
}

impl App {
    pub fn new() -> Self {
        App {
            base_time: Local::now(),
            pomodoro: None,
            state: AppState::Stopped,
        }
    }

    pub fn run(&mut self) {
        self.state = AppState::Running;
        self.start_timer();
    }

    pub fn stop(&mut self) {
        self.state = AppState::Stopped;
        self.pause_timer();
    }

    pub fn toggle_pause(&mut self) {
        match self.state {
            AppState::Stopped => panic!("Error: Can't Pause Stopped App!"),
            AppState::Running => self.pause(),
        }
    }

    fn pause(&mut self) {
        if let Some(p) = &mut self.pomodoro {
            match p.get_state() {
                pomodoro::PomodoroState::Running => self.pause_timer(),
                pomodoro::PomodoroState::Paused => self.start_timer(),
            }
        }
    }

    fn pause_timer(&mut self) {
        if let Some(p) = &mut self.pomodoro {
            p.pause();
        }
    }

    fn start_timer(&mut self) {
        if let Some(p) = &mut self.pomodoro {
            p.run();
        }
    }

    pub fn is_running(&mut self) -> bool {
        self.state == AppState::Running
    }

    pub fn new_pomodoro(&mut self, work_mins: i64, short_break_mins: i64, long_break_mins: i64) {
        self.pomodoro = Some(Pomodoro::new(work_mins, short_break_mins, long_break_mins));
    }

    pub fn update(&mut self) -> io::Result<()> {
        if self.state == AppState::Stopped {
            let err = Error::new(io::ErrorKind::Other, "App Not Running");
            return Err(err);
        }

        self.base_time = Local::now();

        // TODO: Pretty sure there's better syntax for this but I'm blanking right now
        if let Some(p) = &mut self.pomodoro {
            p.update();
        }

        Ok(())
    }

    pub fn get_pomodoro_timer(&self) -> Option<&timer::Timer> {
        match &self.pomodoro {
            Some(p) => Some(p.get_timer()),
            None => None,
        }
    }

    pub fn get_pomodoro_round(&self) -> Option<i32> {
        match &self.pomodoro {
            Some(p) => Some(p.get_round()),
            None => None,
        }
    }
}

impl fmt::Display for App {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match write!(f, "{}\n", self.base_time.format("%H:%M")) {
            Ok(v) => v,
            Err(_e) => panic!("Error Displaying App"),
        };

        match &self.pomodoro {
            Some(p) => match write!(f, "{}", p) {
                Ok(_v) => {}
                Err(_e) => panic!("Error Displaying App"),
            },
            None => match write!(f, "Pomodoro Uninitialized\n") {
                Ok(_v) => {}
                Err(_e) => {}
            },
        }

        {}
        Ok(())
    }
}
