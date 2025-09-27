use crate::rclock::timer;
use chrono::{DateTime, TimeDelta, Utc};
use std::collections::HashMap;
use std::fmt;

struct PomodoroRound {
    work_time: TimeDelta,
    break_time: TimeDelta,
}

pub struct Pomodoro {
    timer: Option<timer::Timer>,
    round_cycle: HashMap<u16, PomodoroRound>,
    round_counter: u16,
    cycle_size: u16,
    intent: Option<TimerIntent>,
    state: PomodoroState,
}

pub enum TimerIntent {
    Work,
    Break,
}

pub enum PomodoroState {
    Running,
    Paused,
}

impl Pomodoro {
    pub fn new(n_rounds: &u16, work_times: &Vec<i64>, break_times: &Vec<i64>) -> Self {
        let mut round_cycle_map: HashMap<u16, PomodoroRound> = HashMap::<u16, PomodoroRound>::new();
        for round in 1..n_rounds + 1 {
            let i: usize = usize::from(round) - 1;
            let work_delta: TimeDelta = TimeDelta::new(work_times[i] * 60, 0).unwrap();
            let break_delta: TimeDelta = TimeDelta::new(break_times[i] * 60, 0).unwrap();
            round_cycle_map.insert(
                round,
                PomodoroRound {
                    work_time: work_delta,
                    break_time: break_delta,
                },
            );
        }

        Self {
            timer: None,
            round_cycle: round_cycle_map,
            round_counter: 1,
            cycle_size: *n_rounds,
            intent: None,
            state: PomodoroState::Paused,
        }
    }

    pub fn init(&mut self) {
        self.round_counter = 1;
        let round = self.round_cycle.get(&self.round_counter).unwrap();
        self.timer = Some(timer::Timer::new(round.work_time));
        self.intent = Some(TimerIntent::Work);
    }

    pub fn run(&mut self) {
        if let None = self.timer {
            self.init();
        }

        self.state = PomodoroState::Running;
        match &mut self.timer {
            Some(t) => t.run(),
            None => panic!("Error: Pomorodor Has No Timer to Run"),
        }
    }

    pub fn pause(&mut self) {
        self.state = PomodoroState::Paused;
        match &mut self.timer {
            Some(t) => t.pause(),
            None => panic!("Error: Pomodoro Has No Timer to Pause"),
        }
    }

    pub fn get_timer(&self) -> Option<&timer::Timer> {
        match &self.timer {
            Some(t) => Some(t),
            None => None,
        }
    }

    pub fn get_round(&self) -> u16 {
        self.round_counter
    }

    pub fn get_state(&self) -> &PomodoroState {
        &self.state
    }

    // TODO: This seems hacky. Probably needs improved
    pub fn get_intent(&self) -> Option<&TimerIntent> {
        match &self.intent {
            Some(i) => Some(&i),
            None => None,
        }
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
                    let round_cycle_idx: u16 = self.round_counter % 4;
                    let round = self.round_cycle.get(&round_cycle_idx).unwrap();
                    self.timer = Some(timer::Timer::new(round.break_time));
                    self.intent = Some(TimerIntent::Break);
                    self.run();
                }
                TimerIntent::Break => {
                    let next_round_num: u16 = self.round_counter + 1;
                    let round_cycle_idx: u16 = (next_round_num % self.cycle_size) + 1;
                    let round: &PomodoroRound = self
                        .round_cycle
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

impl fmt::Display for Pomodoro {
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
