use crate::rclock::app;
use color_eyre::owo_colors::colors::css::MediumSpringGreen;
use ratatui::DefaultTerminal;
use ratatui::Frame;
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

    fn draw(mut self, frame: &mut Frame, msg: String) {
        frame.render_widget(Paragraph::new(msg), frame.area());
    }

    pub fn view(mut self, app: app::App) {
        let time_str = app.base_time.format("%H:%M:%S").to_string();
        self.terminal.draw(|frame| self.draw(frame, time_str));
    }
}
