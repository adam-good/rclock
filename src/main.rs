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
    app.run();

    let mut i = 0;
    while i < 10 && app.is_running() {
        app.update().unwrap();
        app_ui.view(&app);
        app_ui.handle_events(&mut app).expect("UH OH");
        sleep(1);
        i = i + 1;
    }

    app_ui.shutdown(&mut app);
}
