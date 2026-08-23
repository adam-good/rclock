
use rclock::rclock::pomodoro::PomodoroRunner;
use std::time::Duration;

fn main() {
    let work_time = 10;
    let rest_time = 5;
    let work_times = vec![work_time, work_time];
    let rest_times = vec![rest_time, rest_time];
    let runner = PomodoroRunner::new(work_times, rest_times);


}
