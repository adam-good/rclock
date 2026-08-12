

use rclock;

fn main() {
    let timer = rclock::utils::Timer::new();

    std::thread::sleep(std::time::Duration::new(2, 0));

    println!("{:?}", timer.tick());
}
