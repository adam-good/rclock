use std::time::Duration;

mod app;

fn sleep(n: u64) {
    std::thread::sleep(Duration::new(n, 0));
}

fn main() {
    let mut app = app::App::new();

    for _x in vec![1, 2, 3] {
        app.new_timer().unwrap();
        sleep(1);
    }

    let mut i = 0;
    while i < 60 {
        app.update().unwrap();
        println!("{}", app);
        i = i + 1;
        sleep(1);
    }

    println!("{:?}", app.base_time);
}
