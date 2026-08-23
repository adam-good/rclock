use chrono::DateTime;
use chrono::Local;
use std::io;

use std::fmt;

use crate::config::Config;
use crate::rclock::pomodoro;
use crate::rclock::pomodoro::PomodoroRunner;

pub struct App {
    _base_time: DateTime<Local>,
    _pomodoro: Option<pomodoro::PomodoroRunner>,
    _state: _AppState,
    _config: Config,
}

#[derive(Eq, PartialEq)]
enum _AppState {
    _Running, // App is fully running
    _Stopped, // App is completely stopped
}

impl App {
    pub fn new(_config: Config) -> Self {
        unimplemented!()
        //App {
        //    base_time: Local::now(),
        //    pomodoro: None,
        //    state: AppState::Stopped,
        //    config: config,
        //}
    }

    pub fn run(&mut self) {
        unimplemented!()
        //self.state = AppState::Running;
        //self.start_timer();
    }

    pub fn _stop(&mut self) {
        unimplemented!()
        //self.state = AppState::Stopped;
        //self.pause_timer();
    }

    pub fn _toggle_pause(&mut self) {
        unimplemented!()
        //match self.state {
        //    AppState::Stopped => panic!("Error: Can't Pause Stopped App!"),
        //    AppState::Running => self.pause(),
        //}
    }

    fn _pause(&mut self) {
        unimplemented!()
        //if let Some(p) = &mut self.pomodoro {
        //    match p.get_state() {
        //        pomodoro::PomodoroState::Running => self.pause_timer(),
        //        pomodoro::PomodoroState::Paused => self.start_timer(),
        //    }
        //}
    }

    fn _pause_timer(&mut self) {
        unimplemented!()
        //if let Some(p) = &mut self.pomodoro {
        //    p.pause();
        //}
    }

    fn _start_timer(&mut self) {
        unimplemented!()
        //if let Some(p) = &mut self.pomodoro {
        //    p.run();
        //}
    }

    pub fn is_running(&mut self) -> bool {
        unimplemented!()
        //self.state == AppState::Running
    }

    pub fn new_pomodoro(&mut self) {
        unimplemented!()
        //self.pomodoro = Some(PomodoroRunner::new(
        //    &self.config.cycle_len,
        //    &self.config.work_times,
        //    &self.config.break_times,
        //));
    }

    pub fn update(&mut self) -> io::Result<()> {
        unimplemented!()
        //if self.state == AppState::Stopped {
        //    let err = Error::new(io::ErrorKind::Other, "App Not Running");
        //    return Err(err);
        //}

        //self.base_time = Local::now();

        //// TODO: Pretty sure there's better syntax for this but I'm blanking right now
        //if let Some(p) = &mut self.pomodoro {
        //    p.update();
        //}

        //Ok(())
    }

    pub fn _get_pomodoro(&self) -> Option<&PomodoroRunner> {
        unimplemented!()
        //match &self.pomodoro {
        //    Some(p) => Some(&p),
        //    None => None,
        //}
    }
}

impl fmt::Display for App {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
        //match write!(f, "{}\n", self.base_time.format("%H:%M")) {
        //    Ok(v) => v,
        //    Err(_e) => panic!("Error Displaying App"),
        //};

        //match &self.pomodoro {
        //    Some(p) => match write!(f, "{}", p) {
        //        Ok(_v) => {}
        //        Err(_e) => panic!("Error Displaying App"),
        //    },
        //    None => match write!(f, "Pomodoro Uninitialized\n") {
        //        Ok(_v) => {}
        //        Err(_e) => {}
        //    },
        //}

        //{}
        //Ok(())
    }
}
