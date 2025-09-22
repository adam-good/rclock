mod rclock;
use rclock::app;
use rclock::ui;

fn main() {
    let mut app: app::App = app::App::new();
    let mut app_ui: ui::UI = ui::UI::new();

    app.new_pomodoro(3, 1, 2);
    app.run();
    while app.is_running() {
        app.update().unwrap();
        app_ui.view(&app);
        app_ui.handle_events(&mut app).expect("UH OH");
    }

    app_ui.shutdown(&mut app);
}
