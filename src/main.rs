use std::time::Duration;

mod rclock;
use rclock::app;
use rclock::ui;

fn sleep(n: u64) {
    std::thread::sleep(Duration::new(n, 0));
}

fn main() {
    let mut app: app::App = app::App::new();
    let mut app_ui: ui::UI = ui::UI::new();

    app.new_timer().unwrap();

    let mut i = 0;
    while i < 60 {
        app.update().expect("FUCK");
        app_ui.view(&app);
        sleep(1);
        i = i + 1;
    }

    ratatui::restore();

    /*
        let mut i = 0;
        while i < 60 {
            app.update().unwrap();
            println!("{}", app);
            i = i + 1;
            sleep(1);
        }
    */
}
