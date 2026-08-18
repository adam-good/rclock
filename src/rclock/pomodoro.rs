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

    // NOTE: YOU LEFT OFF HERE

    // TODO: This seems hacky. Probably needs improved
    pub fn get_pomo_type(&self) -> PomoType {
        self.current_timer.get_type()
    }

    pub fn update(&mut self) {
        if let Some(timer) = &mut self.timer {
            let default = DateTime::<Utc>::default();
            let _ = timer.update(); //TODO: Should do something here
            if (timer.time() - default).as_seconds_f32() < 0.5 {
                self.cycle_timer();
            }
        }
    }

    fn cycle_timer(&mut self) {
        if let Some(intent) = &self.intent {
            match intent {
                TimerIntent::Work => {
                    let round_cycle_idx: u16 = (self.round_counter % self.cycle_size) + 1;
                    let round = self
                        .schedule
                        .get(&round_cycle_idx)
                        .expect(format!("Can't Find Round for Index {}", round_cycle_idx).as_str());
                    self.timer = Some(timer::Timer::new(round.break_time));
                    self.intent = Some(TimerIntent::Break);
                    self.run();
                }
                TimerIntent::Break => {
                    let next_round_num: u16 = self.round_counter + 1;
                    let round_cycle_idx: u16 = (next_round_num % self.cycle_size) + 1;
                    let round: &PomodoroRound = self
                        .schedule
                        .get(&round_cycle_idx)
                        .expect(format!("Can't find round for {}", round_cycle_idx).as_str());
                    self.timer = Some(timer::Timer::new(round.work_time));
                    self.intent = Some(TimerIntent::Work);
                    self.round_counter = next_round_num;
                    self.run();
                }
            }
        }
    }
}

impl fmt::Display for PomodoroRunner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let timer_str: String = match &self.timer {
            Some(t) => t.to_string(),
            None => "None".to_string(),
        };
        match write!(f, "{{timer: {}\nround: {}}}", timer_str, self.round_counter) {
            Ok(_v) => {}
            Err(_e) => panic!("Error Printing Pomodoro"),
        };

        Ok(())
    }
}
