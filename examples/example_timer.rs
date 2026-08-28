

use std::time::Duration;
use std::thread::sleep;

use rclock::utils::timer::{Timer, GenericTimer};

fn main() {
   let sleep_time = Duration::new(1, 0); 
    let target_duration = Duration::new(60, 0);
    let mut timer = GenericTimer::new_real(target_duration).run();
    for _ in 1..60 {
        timer = timer.tick();
        print!("{}\n", timer);
        sleep(sleep_time);
    }
}
