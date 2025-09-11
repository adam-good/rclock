use crate::rclock::app;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
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
        self.terminal
            .draw(|frame| {
                let (top, left, right) = UI::layout(frame);
                UI::render_top_widget(frame, top, app);
                UI::render_left_widget(frame, left, app);
                UI::render_right_widget(frame, right, app);
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

        (top, left, right)
    }

    // TODO: Make this show current time
    fn render_top_widget(frame: &mut Frame, area: Rect, app: &app::App) {
        let block = Block::new().borders(Borders::ALL);
        let msg = app.base_time.format("%H:%M:%S").to_string();
        frame.render_widget(Paragraph::new(msg).block(block), area);
    }

    // TODO: Make this show a timer
    fn render_left_widget(frame: &mut Frame, area: Rect, app: &app::App) {
        let block = Block::new().borders(Borders::ALL);
        let msg = app.base_time.format("%H:%M:%S").to_string();
        frame.render_widget(Paragraph::new(msg).block(block), area);
    }

    // TODO: Make this show a progress bar
    fn render_right_widget(frame: &mut Frame, area: Rect, app: &app::App) {
        let block = Block::new().borders(Borders::ALL);
        let msg = app.base_time.format("%H:%M:%S").to_string();
        frame.render_widget(Paragraph::new(msg).block(block), area);
    }
}
