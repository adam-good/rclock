

use std::time::Duration;
use std::thread::sleep;

use rclock::utils::timer::Timer;

fn main() {
    let target_duration = Duration::new(60, 0);
    let timer = Timer::new(target_duration).run();
    let sleep_time = Duration::new(1, 0); 
    
    for _ in 1..60 {
        let timer = timer.tick();
        print!("{}\n", timer);
        sleep(sleep_time);
    }
}
