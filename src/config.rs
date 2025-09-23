use std::fs::File;

pub struct Config {
    pub cycle_len: u16,        // Number of rounds per cycle
    pub work_times: Vec<i64>,  // Times (in seconds) of the work timers
    pub break_times: Vec<i64>, // Times (in seconds) of the break timers
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

pub fn load_config_file(file: File) -> Config {
    Config::default()
}

pub fn load_config() -> Config {
    Config::default()
}
