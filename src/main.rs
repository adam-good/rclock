use clap::{Arg, Command};

mod config;
mod rclock;
use config::{Config, load_config};
use rclock::{app, ui};

fn main() {
    let matches = Command::new("rclock")
        .args(vec![
            Arg::new("rounds")
                .short('r')
                .long("rounds")
                .help("How Many Rounds in One Cycle"),
            Arg::new("work")
                .short('w')
                .long("work")
                .help("How Many Minutes on the Work Timer"),
            Arg::new("break")
                .short('b')
                .long("break")
                .help("How Many Minutes on the Break Timer"),
        ])
        .get_matches();

    let config: Config = load_config();

    let mut app: app::App = app::App::new(config);
    let mut app_ui: ui::UI = ui::UI::new();

    app.new_pomodoro();
    app.run();
    while app.is_running() {
        app.update().unwrap();
        app_ui.view(&app);
        app_ui.handle_events(&mut app).expect("UH OH");
    }

    app_ui.shutdown(&mut app);
}
