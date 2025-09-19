use crate::rclock::timer;
use chrono::DateTime;
use chrono::TimeDelta;
use chrono::Utc;
use std::collections::HashMap;
use std::fmt;

struct PomodoroRound {
    work_time: TimeDelta,
    break_time: TimeDelta,
}

pub struct Pomodoro {
    timer: Option<timer::Timer>,
    round_cycle: HashMap<i32, PomodoroRound>,
    round_counter: i32,
    state: PomodoroState,
}

enum PomodoroState {
    Work,
    Break,
}

impl Pomodoro {
    pub fn new(work: i64, short_break: i64, long_break: i64) -> Self {
        let work_delta: TimeDelta =
            TimeDelta::new(work * 60, 0).expect("Failed to Create Work Timer {work}");
        let break_delta1: TimeDelta = TimeDelta::new(short_break * 60, 0)
            .expect("Failed to Create Short Break Timer {short_break}");
        let break_delta2: TimeDelta = TimeDelta::new(long_break * 60, 0)
            .expect("Failed to Create Long Break Timer {long_break}");

        let mut round_cycle_map = HashMap::<i32, PomodoroRound>::new();
        round_cycle_map.insert(
            0,
            PomodoroRound {
                work_time: work_delta,
                break_time: break_delta2,
            },
        );
        round_cycle_map.insert(
            1,
            PomodoroRound {
                work_time: work_delta,
                break_time: break_delta1,
            },
        );
        round_cycle_map.insert(
            2,
            PomodoroRound {
                work_time: work_delta,
                break_time: break_delta1,
            },
        );
        round_cycle_map.insert(
            3,
            PomodoroRound {
                work_time: work_delta,
                break_time: break_delta1,
            },
        );

        let init_timer = timer::Timer::new(work_delta);

        Self {
            timer: Some(init_timer),
            round_cycle: round_cycle_map,
            round_counter: 1,
            state: PomodoroState::Work,
        }
    }

    pub fn run(&mut self) {
        match &mut self.timer {
            Some(t) => t.run(),
            None => panic!("Error: Pomorodor Has No Timer to Run"),
        }
    }

    pub fn pause(&mut self) {
        match &mut self.timer {
            Some(t) => t.pause(),
            None => panic!("Error: Pomodoro Has No Timer to Pause"),
        }
    }

    pub fn get_timer(&self) -> &timer::Timer {
        match &self.timer {
            Some(t) => t,
            None => panic!("Error: Getting None Timer from Pomodoro"),
        }
    }

    pub fn get_round(&self) -> i32 {
        self.round_counter
    }

    pub fn update(&mut self) {
        match &mut self.timer {
            None => panic!("Error: Updating Pomodoro Without a Timer!"),
            Some(timer) => {
                let _ = timer.update();

                if timer.time() - DateTime::<Utc>::default() < TimeDelta::new(0, 1000000).unwrap() {
                    self.cycle_timer();
                }
            }
        }
    }

    fn cycle_timer(&mut self) {
        match self.state {
            PomodoroState::Work => {
                let round_cycle_idx: i32 = self.round_counter % 4;
                let round = self.round_cycle.get(&round_cycle_idx).unwrap();
                self.timer = Some(timer::Timer::new(round.break_time));
                self.state = PomodoroState::Break;
                self.run();
            }
            PomodoroState::Break => {
                let next_round_num: i32 = self.round_counter + 1;
                let round_cycle_idx: i32 = next_round_num % 4;
                let round: &PomodoroRound = self.round_cycle.get(&round_cycle_idx).unwrap();
                self.timer = Some(timer::Timer::new(round.work_time));
                self.state = PomodoroState::Work;
                self.round_counter = next_round_num;
                self.run();
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
        match write!(
            f,
            "{{timer: {}\nround: {}}}",
            timer_str,
            //            self.work_time,
            //            self.short_break_time,
            //           self.long_break_time,
            self.round_counter
        ) {
            Ok(_v) => {}
            Err(_e) => panic!("Error Printing Pomodoro"),
        };

        Ok(())
    }
}
