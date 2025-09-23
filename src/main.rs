mod config;
mod rclock;
use config::{Config, load_config};
use rclock::{app, ui};

fn main() {
    let config: Config = load_config();

    let mut app: app::App = app::App::new();
    let mut app_ui: ui::UI = ui::UI::new();

    app.new_pomodoro(20, 5, 15);
    app.run();
    while app.is_running() {
        app.update().unwrap();
        app_ui.view(&app);
        app_ui.handle_events(&mut app).expect("UH OH");
    }

    app_ui.shutdown(&mut app);
}
