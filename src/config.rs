pub struct Config {
    cycle_len: u16,        // Number of rounds per cycle
    work_times: Vec<u32>,  // Times (in seconds) of the work timers
    break_times: Vec<u32>, // Times (in seconds) of the break timers
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

pub fn load_config() -> Config {
    Config::default()
}
