use crate::rclock::app;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::widgets::Block;
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
                let (top, left, right) = UI::layout(frame);
                frame.render_widget(Paragraph::new(time_str), top);
                frame.render_widget(Paragraph::new("left"), left);
                frame.render_widget(Paragraph::new("right"), right);
            })
            .expect("EEP");
    }

    fn layout(frame: &mut Frame) -> (Rect, Rect, Rect) {
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(frame.area());
        let (top, bottom) = (outer_layout[0], outer_layout[1]);

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(bottom);
        let (left, right) = (inner_layout[0], inner_layout[1]);

        return (top, left, right);
    }
}
