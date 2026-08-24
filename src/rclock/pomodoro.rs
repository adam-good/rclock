use std::fmt;

mod timers;
mod schedule;
use schedule::PomoSchedule;
use timers::{PomoTimer, PomoType};

pub struct PomodoroRunner {
    current_timer: PomoTimer,
    schedule: PomoSchedule,
    round_counter: u16,
    state: PomodoroState,
}

#[derive(Clone,Copy)]
pub enum PomodoroState {
    Running,
    Paused,
}

impl PomodoroRunner {
    pub fn new(work_times: Vec<u64>, break_times: Vec<u64>) -> Self {
        let schedule = PomoSchedule::from_vecs(work_times, break_times);
        Self {
            current_timer: schedule.current_timer(),
            schedule,
            round_counter: 1,
            state: PomodoroState::Paused,
        }
    }

    // NOTE: this will reset the timer according to the record in schedule
    pub fn init(self) -> Self {
        PomodoroRunner { 
            current_timer: self.schedule.current_timer(), 
            schedule: self.schedule, 
            round_counter: self.round_counter, 
            state: PomodoroState::Paused
        }
    }

    pub fn run(self) -> Self {
        PomodoroRunner { 
            current_timer: self.current_timer.run(),
            schedule: self.schedule, 
            round_counter: self.round_counter, 
            state: PomodoroState::Running 
        }
    }

    pub fn pause(self) -> Self {
        PomodoroRunner {
            current_timer: self.current_timer.pause(),
            schedule: self.schedule,
            round_counter: self.round_counter,
            state: PomodoroState::Paused
        }
    }

    pub fn get_timer(&self) -> &PomoTimer {
        &self.current_timer
    }

    pub fn get_round(&self) -> u16 {
        self.round_counter
    }

    pub fn get_state(&self) -> PomodoroState {
        self.state
    }

    pub fn get_pomo_type(&self) -> PomoType {
        self.current_timer.get_type()
    }

    pub fn update(self) -> Self {
        let pomo = if self.current_timer.is_finished() {
            self.advance_schedule().pause()
        } else {
            self
        };

        Self {
            current_timer: pomo.current_timer.update(),
            schedule: pomo.schedule,
            round_counter: pomo.round_counter,
            state: pomo.state,
        }
    }

    pub fn advance_schedule(self) -> Self {
        Self { 
            current_timer: self.current_timer, 
            schedule: self.schedule, 
            round_counter: self.round_counter + 1, 
            state: self.state
        }.init()
    }
}

impl fmt::Display for PomodoroRunner {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
//        let timer_str: String = match &self.timer {
//            Some(t) => t.to_string(),
//            None => "None".to_string(),
//        };
//        match write!(f, "{{timer: {}\nround: {}}}", timer_str, self.round_counter) {
//            Ok(_v) => {}
//            Err(_e) => panic!("Error Printing Pomodoro"),
//        };
//
//        Ok(())
    }
}
