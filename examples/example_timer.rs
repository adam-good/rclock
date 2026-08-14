

use std::time::Duration;
use std::thread::sleep;

use rclock::utils::timer::Timer;

fn main() {
    let timer = Timer::from_now().run();
    let sleep_time = Duration::new(1, 0); 
    loop {
        let timer = timer.tick();
        print!("{}\n", timer);
        sleep(sleep_time);
       
        if timer.duration().as_secs() > 5 {
            break;
        }
    }
}
