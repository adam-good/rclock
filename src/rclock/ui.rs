use crate::rclock::app;
use ratatui::DefaultTerminal;
use ratatui::widgets::Paragraph;

pub struct UI {
    terminal: DefaultTerminal,
}
impl UI {
    pub fn new() -> Self {
        UI {
            terminal: ratatui::init(),
        }
    }

    pub fn view(&mut self, app: &app::App) {
        let time_str = app.base_time.format("%H:%M:%S").to_string();
        self.terminal
            .draw(|frame| {
                frame.render_widget(Paragraph::new(time_str), frame.area());
            })
            .expect("EEP");
    }
}
