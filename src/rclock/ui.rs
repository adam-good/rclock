use crate::rclock::app;
use crate::rclock::drawer;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::layout::Rect;
use ratatui::symbols::Marker;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::canvas::Context;

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

                let (digit1, digit2, sep, digit3, digit4) = UI::clock_layout(top);
                UI::render_clock(frame, digit1, digit2, sep, digit3, digit4);
            })
            .expect("EEP");
    }

    fn layout(frame: &mut Frame) -> (Rect, Rect, Rect) {
        let outer_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(frame.area());
        let (top, bottom) = (outer_layout[0], outer_layout[1]);

        let inner_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(25), Constraint::Percentage(75)])
            .split(bottom);
        let (left, right) = (inner_layout[0], inner_layout[1]);

        (top, left, right)
    }

    fn clock_layout(area: Rect) -> (Rect, Rect, Rect, Rect, Rect) {
        let vert_split = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Percentage(15),
                Constraint::Percentage(50),
                Constraint::Percentage(15),
            ])
            .split(area);
        let middle = vert_split[1];

        let horz_split = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(15),
                Constraint::Percentage(14),
                Constraint::Percentage(14),
                Constraint::Percentage(14),
                Constraint::Percentage(14),
                Constraint::Percentage(14),
                Constraint::Percentage(15),
            ])
            .split(middle);
        let (digit1, digit2, sep, digit3, digit4) = (
            horz_split[1],
            horz_split[2],
            horz_split[3],
            horz_split[4],
            horz_split[5],
        );

        (digit1, digit2, sep, digit3, digit4)
    }

    fn render_clock(
        frame: &mut Frame,
        digit1: Rect,
        digit2: Rect,
        sep: Rect,
        digit3: Rect,
        digit4: Rect,
    ) {
        UI::render_digit(frame, digit1, |context| drawer::zero(context, digit1));
        UI::render_digit(frame, digit2, |context| drawer::zero(context, digit1));
        UI::render_digit(frame, sep, |context| drawer::sep(context, sep));
        UI::render_digit(frame, digit3, |context| drawer::zero(context, digit1));
        UI::render_digit(frame, digit4, |context| drawer::zero(context, digit1));
    }

    fn render_digit(frame: &mut Frame, area: Rect, val: impl Fn(&mut Context)) {
        let left: f64 = 0.0;
        let right: f64 = f64::from(area.width);
        let bottom: f64 = 0.0;
        let top: f64 = f64::from(area.height); //.mul_add(2.0, 0.0);
        // println!("{} {}", right, top);
        let widget = Canvas::default()
            .block(Block::bordered())
            .marker(Marker::HalfBlock)
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(val);

        frame.render_widget(widget, area);
    }

    // TODO: Make this show current time
    fn render_top_widget(frame: &mut Frame, area: Rect, _app: &app::App) {
        let block = Block::new().borders(Borders::ALL);
        //        let text = app.base_time.format("%H:%M:%S").to_string();
        //        let widget = Paragraph::new(text).centered().block(block);
        //        let left: f64 = 0.0;
        //        let right: f64 = f64::from(area.width);
        //        let bottom: f64 = 0.0;
        //        let top = f64::from(area.height).mul_add(2.0, -4.0); // NOTE: Why the mul_add?
        /*
        let widget = Canvas::default()
            .block(block)
            .marker(ratatui::symbols::Marker::HalfBlock)
            .x_bounds([left, right])
            .y_bounds([bottom, top])
            .paint(drawer::zero);
        */

        frame.render_widget(block, area);
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

    /*
        fn center_area(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
            let [area] = Layout::horizontal([horizontal])
                .flex(Flex::Center)
                .areas(area);
            let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);

            area
        }
    */
}
