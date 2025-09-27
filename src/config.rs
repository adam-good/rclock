use serde::{Deserialize, Serialize};
use std::path::Path;

pub struct Config {
    pub cycle_len: u16,        // Number of rounds per cycle
    pub work_times: Vec<i64>,  // Times (in seconds) of the work timers
    pub break_times: Vec<i64>, // Times (in seconds) of the break timers
}

#[derive(Deserialize, Serialize)]
struct TOMLConfig {
    pub cycle_len: Option<u16>,
    pub work_times: Option<Vec<i64>>,
    pub break_times: Option<Vec<i64>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cycle_len: 4,
            work_times: vec![20, 20, 20, 20],
            break_times: vec![5, 5, 5, 15],
        }
    }
}

impl Config {
    pub fn from_matches(mut self, matches: clap::ArgMatches) -> Self {
        if let Some(num_rounds) = matches.get_one::<u16>("rounds") {
            self.cycle_len = *num_rounds;
        }
        if let Some(work_times) = matches.get_one::<Vec<i64>>("work") {
            self.work_times = work_times.to_owned(); //TODO: is this okay?
        }
        if let Some(break_times) = matches.get_one::<Vec<i64>>("break") {
            self.break_times = break_times.to_owned()
        }

        self
    }

    pub fn from_config(mut self, path_str: String) -> Self {
        let path = Path::new(path_str.as_str());
        match path.exists() {
            false => self,
            true => {
                let data_str: String =
                    std::fs::read_to_string(path).expect("Failed to Read Config File");
                let toml_config: TOMLConfig =
                    toml::from_str(&data_str).expect("Failed to Parse TOML");

                if let Some(rounds) = toml_config.cycle_len {
                    self.cycle_len = rounds;
                }
                if let Some(work_times) = toml_config.work_times {
                    self.work_times = work_times;
                }
                if let Some(break_times) = toml_config.break_times {
                    self.break_times = break_times
                }

                self
            }
        }
    }
}
